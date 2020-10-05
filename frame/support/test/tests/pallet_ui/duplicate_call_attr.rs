#[frame_support::pallet]
mod pallet {
	use frame_support::pallet_prelude::Interface;
	use frame_system::pallet_prelude::BlockNumberFor;
	use frame_support::pallet_prelude::StorageValueType;

	#[pallet::config]
	pub trait Config: frame_system::Config {}

	#[pallet::pallet]
	#[pallet::generate_store(trait Store)]
	pub struct Pallet<T>(core::marker::PhantomData<T>);

	#[pallet::interface]
	impl<T: Config> Interface<BlockNumberFor<T>> for Pallet<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	#[pallet::storage]
	type Foo<T> = StorageValueType<_, u8>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {}
}

fn main() {
}