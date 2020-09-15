// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use super::helper;
use syn::spanned::Spanned;

/// Definition of  type value. Just a function which is expanded to a struct implementing `Get`.
/// e.g.: `fn foo<T: Trait>() -> T::Balance { T::Balance::zero() }`
pub struct TypeValueDef {
	/// The index of error item in pallet module.
	pub index: usize,
	/// Visibility of the struct to generate.
	pub vis: syn::Visibility,
	/// Ident of the struct to generate.
	pub ident: syn::Ident,
	/// The type return by Get.
	pub type_: Box<syn::Type>,
	/// The block returning the value to get
	pub block: Box<syn::Block>,
	/// If type value is generic over trait T.
	pub has_trait: bool,
	/// If type value is generic over instance I.
	pub has_instance: bool,
	/// A set of usage of instance, must be check for consistency with trait.
	pub instances: Vec<helper::InstanceUsage>,
}

impl TypeValueDef {
	pub fn try_from(index: usize, item: &mut syn::Item) -> syn::Result<Self> {
		let item = if let syn::Item::Fn(item) = item {
			item
		} else {
			return Err(syn::Error::new(item.span(), "Invalid pallet::type_value, expect item fn"));
		};


		if !item.attrs.is_empty() {
			let msg = "Invalid pallet::type_value, unexpected attribute";
			return Err(syn::Error::new(item.attrs[0].span(), msg));
		}

		if let Some(span) = item.sig.constness.as_ref().map(|t| t.span())
			.or(item.sig.asyncness.as_ref().map(|t| t.span()))
			.or(item.sig.unsafety.as_ref().map(|t| t.span()))
			.or(item.sig.abi.as_ref().map(|t| t.span()))
			.or(item.sig.variadic.as_ref().map(|t| t.span()))
		{
			let msg = "Invalid pallet::type_value, unexpected token";
			return Err(syn::Error::new(span, msg));
		}

		if !item.sig.inputs.is_empty() {
			let msg = "Invalid pallet::type_value, unexpected argument";
			return Err(syn::Error::new(item.sig.inputs[0].span(), msg));
		}

		let vis = item.vis.clone();
		let ident = item.sig.ident.clone();
		let block = item.block.clone();
		let type_ = match item.sig.output.clone() {
			syn::ReturnType::Type(_, type_) => type_,
			syn::ReturnType::Default => {
				let msg = "Invalid pallet::type_value, expect return type";
				return Err(syn::Error::new(item.sig.span(), msg));
			},
		};

		let mut instances = vec![];
		if let Some(usage) = helper::check_type_value_gen(&item.sig.generics, item.sig.span())? {
			instances.push(usage);
		}

		let has_instance = item.sig.generics.type_params().any(|t| t.ident == "I");
		let has_trait = item.sig.generics.type_params().any(|t| t.ident == "T");

		Ok(TypeValueDef {
			index,
			has_trait,
			has_instance,
			vis,
			ident,
			block,
			type_,
			instances,
		})
	}
}