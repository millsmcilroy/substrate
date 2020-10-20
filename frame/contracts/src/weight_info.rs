// Copyright 2020 Parity Technologies (UK) Ltd.
// This file is part of Substrate.

// Substrate is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Substrate is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Substrate. If not, see <http://www.gnu.org/licenses/>.

//! This module contains the `WeightInfo` trait and its unsafe implementation on `()`.

use frame_support::weights::{Weight, constants::RocksDbWeight as DbWeight};

/// Should be implemented by automatically generated code of the benchmarking system for
/// every runtime that makes use of this pallet.
/// This trait is also implemented on `()`. The implemention on `()` is **unsafe** and must
/// only be used during development. Proper weights can be generated by running the
/// pallet_contracts benchmark suite for the runtime in question.
pub trait WeightInfo {
	fn update_schedule() -> Weight;
	fn put_code(n: u32, ) -> Weight;
	fn instantiate(n: u32, ) -> Weight;
	fn call() -> Weight;
	fn claim_surcharge() -> Weight;
	fn seal_caller(r: u32, ) -> Weight;
	fn seal_address(r: u32, ) -> Weight;
	fn seal_gas_left(r: u32, ) -> Weight;
	fn seal_balance(r: u32, ) -> Weight;
	fn seal_value_transferred(r: u32, ) -> Weight;
	fn seal_minimum_balance(r: u32, ) -> Weight;
	fn seal_tombstone_deposit(r: u32, ) -> Weight;
	fn seal_rent_allowance(r: u32, ) -> Weight;
	fn seal_block_number(r: u32, ) -> Weight;
	fn seal_now(r: u32, ) -> Weight;
	fn seal_weight_to_fee(r: u32, ) -> Weight;
	fn seal_gas(r: u32, ) -> Weight;
	fn seal_input(r: u32, ) -> Weight;
	fn seal_input_per_kb(n: u32, ) -> Weight;
	fn seal_return(r: u32, ) -> Weight;
	fn seal_return_per_kb(n: u32, ) -> Weight;
	fn seal_terminate(r: u32, ) -> Weight;
	fn seal_restore_to(r: u32, ) -> Weight;
	fn seal_restore_to_per_delta(d: u32, ) -> Weight;
	fn seal_random(r: u32, ) -> Weight;
	fn seal_deposit_event(r: u32, ) -> Weight;
	fn seal_deposit_event_per_topic_and_kb(t: u32, n: u32, ) -> Weight;
	fn seal_set_rent_allowance(r: u32, ) -> Weight;
	fn seal_set_storage(r: u32, ) -> Weight;
	fn seal_set_storage_per_kb(n: u32, ) -> Weight;
	fn seal_clear_storage(r: u32, ) -> Weight;
	fn seal_get_storage(r: u32, ) -> Weight;
	fn seal_get_storage_per_kb(n: u32, ) -> Weight;
	fn seal_transfer(r: u32, ) -> Weight;
	fn seal_call(r: u32, ) -> Weight;
	fn seal_call_per_transfer_input_output_kb(t: u32, i: u32, o: u32, ) -> Weight;
	fn seal_instantiate(r: u32, ) -> Weight;
	fn seal_instantiate_per_input_output_kb(i: u32, o: u32, ) -> Weight;
	fn seal_hash_sha2_256(r: u32, ) -> Weight;
	fn seal_hash_sha2_256_per_kb(n: u32, ) -> Weight;
	fn seal_hash_keccak_256(r: u32, ) -> Weight;
	fn seal_hash_keccak_256_per_kb(n: u32, ) -> Weight;
	fn seal_hash_blake2_256(r: u32, ) -> Weight;
	fn seal_hash_blake2_256_per_kb(n: u32, ) -> Weight;
	fn seal_hash_blake2_128(r: u32, ) -> Weight;
	fn seal_hash_blake2_128_per_kb(n: u32, ) -> Weight;
	fn instr_i64const(r: u32, ) -> Weight;
	fn instr_i64load(r: u32, ) -> Weight;
	fn instr_i64store(r: u32, ) -> Weight;
	fn instr_select(r: u32, ) -> Weight;
	fn instr_if(r: u32, ) -> Weight;
	fn instr_br(r: u32, ) -> Weight;
	fn instr_br_if(r: u32, ) -> Weight;
	fn instr_br_table(r: u32, ) -> Weight;
	fn instr_br_table_per_entry(e: u32, ) -> Weight;
	fn instr_call(r: u32, ) -> Weight;
	fn instr_call_indirect(r: u32, ) -> Weight;
	fn instr_call_indirect_per_param(p: u32, ) -> Weight;
	fn instr_local_get(r: u32, ) -> Weight;
	fn instr_local_set(r: u32, ) -> Weight;
	fn instr_local_tee(r: u32, ) -> Weight;
	fn instr_global_get(r: u32, ) -> Weight;
	fn instr_global_set(r: u32, ) -> Weight;
	fn instr_memory_current(r: u32, ) -> Weight;
	fn instr_memory_grow(r: u32, ) -> Weight;
	fn instr_i64clz(r: u32, ) -> Weight;
	fn instr_i64ctz(r: u32, ) -> Weight;
	fn instr_i64popcnt(r: u32, ) -> Weight;
	fn instr_i64eqz(r: u32, ) -> Weight;
	fn instr_i64extendsi32(r: u32, ) -> Weight;
	fn instr_i64extendui32(r: u32, ) -> Weight;
	fn instr_i32wrapi64(r: u32, ) -> Weight;
	fn instr_i64eq(r: u32, ) -> Weight;
	fn instr_i64ne(r: u32, ) -> Weight;
	fn instr_i64lts(r: u32, ) -> Weight;
	fn instr_i64ltu(r: u32, ) -> Weight;
	fn instr_i64gts(r: u32, ) -> Weight;
	fn instr_i64gtu(r: u32, ) -> Weight;
	fn instr_i64les(r: u32, ) -> Weight;
	fn instr_i64leu(r: u32, ) -> Weight;
	fn instr_i64ges(r: u32, ) -> Weight;
	fn instr_i64geu(r: u32, ) -> Weight;
	fn instr_i64add(r: u32, ) -> Weight;
	fn instr_i64sub(r: u32, ) -> Weight;
	fn instr_i64mul(r: u32, ) -> Weight;
	fn instr_i64divs(r: u32, ) -> Weight;
	fn instr_i64divu(r: u32, ) -> Weight;
	fn instr_i64rems(r: u32, ) -> Weight;
	fn instr_i64remu(r: u32, ) -> Weight;
	fn instr_i64and(r: u32, ) -> Weight;
	fn instr_i64or(r: u32, ) -> Weight;
	fn instr_i64xor(r: u32, ) -> Weight;
	fn instr_i64shl(r: u32, ) -> Weight;
	fn instr_i64shrs(r: u32, ) -> Weight;
	fn instr_i64shru(r: u32, ) -> Weight;
	fn instr_i64rotl(r: u32, ) -> Weight;
	fn instr_i64rotr(r: u32, ) -> Weight;
}

/// Unsafe implementation that must only be used for development.
impl WeightInfo for () {
	fn update_schedule() -> Weight {
		(31_605_000 as Weight)
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn put_code(n: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((146_455_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(1 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn instantiate(n: u32, ) -> Weight {
		(210_976_000 as Weight)
			.saturating_add((1_011_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().writes(3 as Weight))
	}
	fn call() -> Weight {
		(198_863_000 as Weight)
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn claim_surcharge() -> Weight {
		(526_353_000 as Weight)
			.saturating_add(DbWeight::get().reads(3 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn seal_caller(r: u32, ) -> Weight {
		(141_323_000 as Weight)
			.saturating_add((371_484_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_address(r: u32, ) -> Weight {
		(140_767_000 as Weight)
			.saturating_add((372_057_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_gas_left(r: u32, ) -> Weight {
		(135_810_000 as Weight)
			.saturating_add((369_355_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_balance(r: u32, ) -> Weight {
		(126_565_000 as Weight)
			.saturating_add((814_186_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_value_transferred(r: u32, ) -> Weight {
		(145_397_000 as Weight)
			.saturating_add((366_417_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_minimum_balance(r: u32, ) -> Weight {
		(143_113_000 as Weight)
			.saturating_add((367_922_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_tombstone_deposit(r: u32, ) -> Weight {
		(142_130_000 as Weight)
			.saturating_add((368_505_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_rent_allowance(r: u32, ) -> Weight {
		(170_396_000 as Weight)
			.saturating_add((841_570_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_block_number(r: u32, ) -> Weight {
		(146_827_000 as Weight)
			.saturating_add((366_457_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_now(r: u32, ) -> Weight {
		(140_029_000 as Weight)
			.saturating_add((368_912_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_weight_to_fee(r: u32, ) -> Weight {
		(146_650_000 as Weight)
			.saturating_add((609_802_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_gas(r: u32, ) -> Weight {
		(123_587_000 as Weight)
			.saturating_add((185_957_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_input(r: u32, ) -> Weight {
		(135_749_000 as Weight)
			.saturating_add((7_858_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_input_per_kb(n: u32, ) -> Weight {
		(146_791_000 as Weight)
			.saturating_add((272_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_return(r: u32, ) -> Weight {
		(124_496_000 as Weight)
			.saturating_add((5_812_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_return_per_kb(n: u32, ) -> Weight {
		(135_291_000 as Weight)
			.saturating_add((666_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_terminate(r: u32, ) -> Weight {
		(135_262_000 as Weight)
			.saturating_add((360_888_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((2 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes((3 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_restore_to(r: u32, ) -> Weight {
		(231_683_000 as Weight)
			.saturating_add((134_947_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((3 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes((4 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_restore_to_per_delta(d: u32, ) -> Weight {
		(330_926_000 as Weight)
			.saturating_add((3_724_061_000 as Weight).saturating_mul(d as Weight))
			.saturating_add(DbWeight::get().reads(7 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(d as Weight)))
			.saturating_add(DbWeight::get().writes(5 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(d as Weight)))
	}
	fn seal_random(r: u32, ) -> Weight {
		(151_408_000 as Weight)
			.saturating_add((945_807_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_deposit_event(r: u32, ) -> Weight {
		(144_531_000 as Weight)
			.saturating_add((1_342_349_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_deposit_event_per_topic_and_kb(t: u32, n: u32, ) -> Weight {
		(1_763_597_000 as Weight)
			.saturating_add((747_998_000 as Weight).saturating_mul(t as Weight))
			.saturating_add((240_986_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(t as Weight)))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(t as Weight)))
	}
	fn seal_set_rent_allowance(r: u32, ) -> Weight {
		(170_874_000 as Weight)
			.saturating_add((982_881_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().writes(1 as Weight))
	}
	fn seal_set_storage(r: u32, ) -> Weight {
		(2_319_579_000 as Weight)
			.saturating_add((14_948_147_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_set_storage_per_kb(n: u32, ) -> Weight {
		(2_297_742_000 as Weight)
			.saturating_add((209_576_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().writes(2 as Weight))
	}
	fn seal_clear_storage(r: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((5_072_853_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_get_storage(r: u32, ) -> Weight {
		(99_710_000 as Weight)
			.saturating_add((1_076_292_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_get_storage_per_kb(n: u32, ) -> Weight {
		(879_798_000 as Weight)
			.saturating_add((145_234_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
	}
	fn seal_transfer(r: u32, ) -> Weight {
		(73_025_000 as Weight)
			.saturating_add((6_172_151_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(1 as Weight))
			.saturating_add(DbWeight::get().writes((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_call(r: u32, ) -> Weight {
		(117_773_000 as Weight)
			.saturating_add((9_950_452_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(5 as Weight))
			.saturating_add(DbWeight::get().reads((100 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_call_per_transfer_input_output_kb(t: u32, i: u32, o: u32, ) -> Weight {
		(11_764_452_000 as Weight)
			.saturating_add((4_836_973_000 as Weight).saturating_mul(t as Weight))
			.saturating_add((52_285_000 as Weight).saturating_mul(i as Weight))
			.saturating_add((72_745_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(105 as Weight))
			.saturating_add(DbWeight::get().reads((101 as Weight).saturating_mul(t as Weight)))
			.saturating_add(DbWeight::get().writes((101 as Weight).saturating_mul(t as Weight)))
	}
	fn seal_instantiate(r: u32, ) -> Weight {
		(0 as Weight)
			.saturating_add((21_630_734_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(6 as Weight))
			.saturating_add(DbWeight::get().reads((300 as Weight).saturating_mul(r as Weight)))
			.saturating_add(DbWeight::get().writes(2 as Weight))
			.saturating_add(DbWeight::get().writes((200 as Weight).saturating_mul(r as Weight)))
	}
	fn seal_instantiate_per_input_output_kb(i: u32, o: u32, ) -> Weight {
		(19_803_069_000 as Weight)
			.saturating_add((154_331_000 as Weight).saturating_mul(i as Weight))
			.saturating_add((74_314_000 as Weight).saturating_mul(o as Weight))
			.saturating_add(DbWeight::get().reads(207 as Weight))
			.saturating_add(DbWeight::get().writes(202 as Weight))
	}
	fn seal_hash_sha2_256(r: u32, ) -> Weight {
		(137_931_000 as Weight)
			.saturating_add((318_016_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_sha2_256_per_kb(n: u32, ) -> Weight {
		(138_157_000 as Weight)
			.saturating_add((425_649_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_keccak_256(r: u32, ) -> Weight {
		(140_557_000 as Weight)
			.saturating_add((331_088_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_keccak_256_per_kb(n: u32, ) -> Weight {
		(511_354_000 as Weight)
			.saturating_add((335_283_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_256(r: u32, ) -> Weight {
		(131_655_000 as Weight)
			.saturating_add((307_658_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_256_per_kb(n: u32, ) -> Weight {
		(635_740_000 as Weight)
			.saturating_add((153_568_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_128(r: u32, ) -> Weight {
		(134_846_000 as Weight)
			.saturating_add((306_403_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn seal_hash_blake2_128_per_kb(n: u32, ) -> Weight {
		(556_612_000 as Weight)
			.saturating_add((153_220_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(DbWeight::get().reads(4 as Weight))
	}
	fn instr_i64const(r: u32, ) -> Weight {
		(26_216_000 as Weight)
			.saturating_add((3_533_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64load(r: u32, ) -> Weight {
		(28_700_000 as Weight)
			.saturating_add((162_830_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64store(r: u32, ) -> Weight {
		(28_725_000 as Weight)
			.saturating_add((233_387_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_select(r: u32, ) -> Weight {
		(26_138_000 as Weight)
			.saturating_add((13_704_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_if(r: u32, ) -> Weight {
		(26_112_000 as Weight)
			.saturating_add((13_148_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br(r: u32, ) -> Weight {
		(26_136_000 as Weight)
			.saturating_add((6_574_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_if(r: u32, ) -> Weight {
		(26_134_000 as Weight)
			.saturating_add((15_068_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_table(r: u32, ) -> Weight {
		(26_151_000 as Weight)
			.saturating_add((16_770_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_br_table_per_entry(e: u32, ) -> Weight {
		(40_194_000 as Weight)
			.saturating_add((99_000 as Weight).saturating_mul(e as Weight))
	}
	fn instr_call(r: u32, ) -> Weight {
		(26_472_000 as Weight)
			.saturating_add((98_902_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_call_indirect(r: u32, ) -> Weight {
		(34_361_000 as Weight)
			.saturating_add((204_262_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_call_indirect_per_param(p: u32, ) -> Weight {
		(255_515_000 as Weight)
			.saturating_add((3_885_000 as Weight).saturating_mul(p as Weight))
	}
	fn instr_local_get(r: u32, ) -> Weight {
		(45_063_000 as Weight)
			.saturating_add((3_531_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_local_set(r: u32, ) -> Weight {
		(44_967_000 as Weight)
			.saturating_add((3_777_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_local_tee(r: u32, ) -> Weight {
		(44_997_000 as Weight)
			.saturating_add((5_251_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_global_get(r: u32, ) -> Weight {
		(29_656_000 as Weight)
			.saturating_add((8_268_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_global_set(r: u32, ) -> Weight {
		(29_609_000 as Weight)
			.saturating_add((12_134_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_memory_current(r: u32, ) -> Weight {
		(28_564_000 as Weight)
			.saturating_add((3_958_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_memory_grow(r: u32, ) -> Weight {
		(26_916_000 as Weight)
			.saturating_add((1_859_826_370_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64clz(r: u32, ) -> Weight {
		(26_205_000 as Weight)
			.saturating_add((5_616_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ctz(r: u32, ) -> Weight {
		(26_164_000 as Weight)
			.saturating_add((5_469_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64popcnt(r: u32, ) -> Weight {
		(26_133_000 as Weight)
			.saturating_add((6_172_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64eqz(r: u32, ) -> Weight {
		(26_089_000 as Weight)
			.saturating_add((6_595_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64extendsi32(r: u32, ) -> Weight {
		(26_076_000 as Weight)
			.saturating_add((6_135_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64extendui32(r: u32, ) -> Weight {
		(26_108_000 as Weight)
			.saturating_add((6_142_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i32wrapi64(r: u32, ) -> Weight {
		(26_105_000 as Weight)
			.saturating_add((6_120_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64eq(r: u32, ) -> Weight {
		(26_135_000 as Weight)
			.saturating_add((7_742_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ne(r: u32, ) -> Weight {
		(26_083_000 as Weight)
			.saturating_add((7_524_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64lts(r: u32, ) -> Weight {
		(26_114_000 as Weight)
			.saturating_add((7_447_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ltu(r: u32, ) -> Weight {
		(26_113_000 as Weight)
			.saturating_add((7_571_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64gts(r: u32, ) -> Weight {
		(26_066_000 as Weight)
			.saturating_add((7_532_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64gtu(r: u32, ) -> Weight {
		(26_069_000 as Weight)
			.saturating_add((7_408_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64les(r: u32, ) -> Weight {
		(26_056_000 as Weight)
			.saturating_add((7_499_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64leu(r: u32, ) -> Weight {
		(26_079_000 as Weight)
			.saturating_add((7_573_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64ges(r: u32, ) -> Weight {
		(26_071_000 as Weight)
			.saturating_add((7_772_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64geu(r: u32, ) -> Weight {
		(26_055_000 as Weight)
			.saturating_add((7_674_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64add(r: u32, ) -> Weight {
		(26_080_000 as Weight)
			.saturating_add((7_701_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64sub(r: u32, ) -> Weight {
		(26_043_000 as Weight)
			.saturating_add((7_505_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64mul(r: u32, ) -> Weight {
		(26_095_000 as Weight)
			.saturating_add((7_399_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64divs(r: u32, ) -> Weight {
		(26_047_000 as Weight)
			.saturating_add((13_273_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64divu(r: u32, ) -> Weight {
		(26_054_000 as Weight)
			.saturating_add((12_226_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rems(r: u32, ) -> Weight {
		(26_091_000 as Weight)
			.saturating_add((13_183_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64remu(r: u32, ) -> Weight {
		(26_075_000 as Weight)
			.saturating_add((12_196_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64and(r: u32, ) -> Weight {
		(26_109_000 as Weight)
			.saturating_add((7_460_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64or(r: u32, ) -> Weight {
		(26_091_000 as Weight)
			.saturating_add((7_517_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64xor(r: u32, ) -> Weight {
		(26_136_000 as Weight)
			.saturating_add((7_498_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shl(r: u32, ) -> Weight {
		(26_156_000 as Weight)
			.saturating_add((7_695_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shrs(r: u32, ) -> Weight {
		(26_157_000 as Weight)
			.saturating_add((7_447_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64shru(r: u32, ) -> Weight {
		(26_120_000 as Weight)
			.saturating_add((7_659_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rotl(r: u32, ) -> Weight {
		(26_104_000 as Weight)
			.saturating_add((7_688_000 as Weight).saturating_mul(r as Weight))
	}
	fn instr_i64rotr(r: u32, ) -> Weight {
		(26_090_000 as Weight)
			.saturating_add((7_619_000 as Weight).saturating_mul(r as Weight))
	}
}
