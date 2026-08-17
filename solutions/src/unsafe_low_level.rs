//! Capítulos 44 a 48: wrappers safe, layout, FFI conceptual e inicialización.

pub mod c44 {
    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RawSliceFacts {
        pub non_null_even_when_empty: bool,
        pub properly_aligned: bool,
        pub contained_in_one_allocation: bool,
        pub initialized_for_element_type: bool,
        pub readable_for_returned_lifetime: bool,
        pub no_conflicting_mutation: bool,
        pub byte_len_within_isize: bool,
        pub address_addition_does_not_wrap: bool,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum RawSliceViolation {
        Null,
        Misaligned,
        MultipleAllocations,
        Uninitialized,
        NotReadableForLifetime,
        ConflictingMutation,
        TooLarge,
        AddressWrap,
    }

    // SOLUTION: C44-E01
    pub const fn audit_raw_slice_contract(facts: RawSliceFacts) -> Result<(), RawSliceViolation> {
        if !facts.non_null_even_when_empty {
            return Err(RawSliceViolation::Null);
        }
        if !facts.properly_aligned {
            return Err(RawSliceViolation::Misaligned);
        }
        if !facts.contained_in_one_allocation {
            return Err(RawSliceViolation::MultipleAllocations);
        }
        if !facts.initialized_for_element_type {
            return Err(RawSliceViolation::Uninitialized);
        }
        if !facts.readable_for_returned_lifetime {
            return Err(RawSliceViolation::NotReadableForLifetime);
        }
        if !facts.no_conflicting_mutation {
            return Err(RawSliceViolation::ConflictingMutation);
        }
        if !facts.byte_len_within_isize {
            return Err(RawSliceViolation::TooLarge);
        }
        if !facts.address_addition_does_not_wrap {
            return Err(RawSliceViolation::AddressWrap);
        }
        Ok(())
    }

    /// Reads one copyable value from a raw pointer.
    ///
    /// # Safety
    ///
    /// `pointer` must be non-null, properly aligned, initialized for `T` and
    /// valid for a read during this call. No conflicting access may occur.
    // SOLUTION: C44-E02
    pub unsafe fn read_one<T: Copy>(pointer: *const T) -> T {
        // SAFETY: the caller provides every precondition required by `read`.
        unsafe { pointer.read() }
    }

    // SOLUTION: C44-E03
    pub fn replace_after_successful_build<T, F>(slot: &mut T, build: F)
    where
        F: FnOnce(&T) -> T,
    {
        let replacement = build(slot);
        *slot = replacement;
    }

    unsafe fn raw_sum(pointer: *const i32, length: usize) -> i32 {
        // SAFETY: el caller promete una región legible de `length` elementos.
        let values = unsafe { std::slice::from_raw_parts(pointer, length) };
        values.iter().sum()
    }

    // SOLUTION: C44-E04
    pub fn safe_sum(values: &[i32]) -> i32 {
        // SAFETY: `as_ptr` y `len` proceden del mismo slice vivo e inicializado.
        unsafe { raw_sum(values.as_ptr(), values.len()) }
    }

    /// Describes storage that a safe consumer may view as contiguous bytes.
    ///
    /// # Safety
    ///
    /// `raw_parts` must return a non-null pointer and a byte length contained
    /// in one live allocation. The range must stay initialized and readable,
    /// without conflicting mutation, for the lifetime of `&self`.
    pub unsafe trait ContiguousBytes {
        fn raw_parts(&self) -> (*const u8, usize);
    }

    // SAFETY: an array owns exactly `LENGTH` initialized contiguous bytes;
    // its shared borrow prevents conflicting safe mutation.
    unsafe impl<const LENGTH: usize> ContiguousBytes for [u8; LENGTH] {
        fn raw_parts(&self) -> (*const u8, usize) {
            (self.as_ptr(), self.len())
        }
    }

    // SOLUTION: C44-E05
    pub fn byte_sum<T>(value: &T) -> u64
    where
        T: ContiguousBytes + ?Sized,
    {
        let (pointer, length) = value.raw_parts();
        // SAFETY: `ContiguousBytes` makes these facts an implementer obligation,
        // and the returned slice cannot outlive the shared borrow of `value`.
        let bytes = unsafe { std::slice::from_raw_parts(pointer, length) };
        bytes.iter().map(|byte| u64::from(*byte)).sum()
    }

    // SOLUTION: C44-E06
    pub fn checked_get<T>(values: &[T], index: usize) -> Option<&T> {
        if index < values.len() {
            // SAFETY: the branch proves `index < values.len()` for this slice.
            Some(unsafe { values.get_unchecked(index) })
        } else {
            None
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FfiSliceError {
        NullWithElements,
    }

    /// Sums a pointer-length pair received through an FFI-like boundary.
    ///
    /// # Safety
    ///
    /// If `pointer` is non-null, it must satisfy every precondition of
    /// `slice::from_raw_parts(pointer, length)` for the duration of this call.
    /// A null pointer is accepted and returns `Ok(0)` only when `length == 0`.
    // SOLUTION: C44-E07
    pub unsafe fn ffi_sum(pointer: *const i32, length: usize) -> Result<i32, FfiSliceError> {
        if pointer.is_null() {
            return if length == 0 {
                Ok(0)
            } else {
                Err(FfiSliceError::NullWithElements)
            };
        }

        // SAFETY: the non-null case inherits the remaining requirements from
        // the public safety contract; the slice is used only during this call.
        let values = unsafe { std::slice::from_raw_parts(pointer, length) };
        Ok(values.iter().sum())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::panic::{AssertUnwindSafe, catch_unwind};

        fn valid_raw_slice_facts() -> RawSliceFacts {
            RawSliceFacts {
                non_null_even_when_empty: true,
                properly_aligned: true,
                contained_in_one_allocation: true,
                initialized_for_element_type: true,
                readable_for_returned_lifetime: true,
                no_conflicting_mutation: true,
                byte_len_within_isize: true,
                address_addition_does_not_wrap: true,
            }
        }

        #[test]
        fn raw_slice_contract_requires_every_independent_fact() {
            assert_eq!(audit_raw_slice_contract(valid_raw_slice_facts()), Ok(()));

            let mut facts = valid_raw_slice_facts();
            facts.contained_in_one_allocation = false;
            assert_eq!(
                audit_raw_slice_contract(facts),
                Err(RawSliceViolation::MultipleAllocations),
            );

            let mut facts = valid_raw_slice_facts();
            facts.non_null_even_when_empty = false;
            assert_eq!(
                audit_raw_slice_contract(facts),
                Err(RawSliceViolation::Null),
            );
        }

        #[test]
        fn unsafe_function_consumes_its_documented_caller_contract() {
            let value = 42;
            // SAFETY: the pointer comes from a live, aligned, initialized `i32`.
            assert_eq!(unsafe { read_one(&raw const value) }, 42);
        }

        #[test]
        fn build_before_replace_keeps_the_old_value_when_callback_panics() {
            let mut value = String::from("valid before");
            let result = catch_unwind(AssertUnwindSafe(|| {
                replace_after_successful_build(&mut value, |_| panic!("builder failed"));
            }));

            assert!(result.is_err());
            assert_eq!(value, "valid before");
        }

        #[test]
        fn safe_boundary_constructs_coherent_pointer_and_length() {
            assert_eq!(safe_sum(&[10, 20, 12]), 42);
            assert_eq!(safe_sum(&[]), 0);
        }

        #[test]
        fn unsafe_trait_moves_the_proof_to_each_implementation() {
            assert_eq!(byte_sum(&[1_u8, 2, 3, 4]), 10);
            assert_eq!(byte_sum(&[]), 0);
        }

        #[test]
        fn checked_wrapper_never_calls_unchecked_access_out_of_bounds() {
            let values = [10, 20];
            assert_eq!(checked_get(&values, 1), Some(&20));
            assert_eq!(checked_get(&values, 2), None);
            assert_eq!(checked_get::<i32>(&[], 0), None);
        }

        #[test]
        fn raw_pointer_boundary_stays_unsafe_after_checking_null() {
            let values = [10, 20, 12];
            // SAFETY: pointer and length come from the same live array.
            assert_eq!(unsafe { ffi_sum(values.as_ptr(), values.len()) }, Ok(42));
            // SAFETY: the function explicitly accepts null when length is zero.
            assert_eq!(unsafe { ffi_sum(std::ptr::null(), 0) }, Ok(0));
            // SAFETY: this branch rejects null before attempting any memory access.
            assert_eq!(
                unsafe { ffi_sum(std::ptr::null(), 3) },
                Err(FfiSliceError::NullWithElements),
            );
        }
    }
}

pub mod c45 {
    use std::cell::UnsafeCell;
    use std::marker::PhantomData;
    use std::mem::align_of;
    use std::ops::Range;
    use std::ptr::{self, NonNull};

    /// Reads one copyable element from a caller-owned raw region.
    ///
    /// # Safety
    ///
    /// When `index < length`, `pointer` must be derived from one live
    /// allocation that contains at least `length` initialized `T` values. It
    /// must be properly aligned and valid for a read at `index`, and no access
    /// incompatible with that read may occur during this call.
    // SOLUTION: C45-E01
    pub unsafe fn read_copy_at<T: Copy>(
        pointer: *const T,
        length: usize,
        index: usize,
    ) -> Option<T> {
        if index >= length {
            return None;
        }

        // SAFETY: the branch proves the index is in the caller-provided region;
        // the remaining access requirements are the public safety contract.
        Some(unsafe { pointer.add(index).read() })
    }

    // SOLUTION: C45-E02
    pub fn aligned_but_dangling<T>() -> NonNull<T> {
        NonNull::dangling()
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct InsufficientTagAlignment;

    pub struct TaggedRef<'a, T> {
        tagged: *const T,
        owner: PhantomData<&'a T>,
    }

    // SOLUTION: C45-E03
    impl<'a, T> TaggedRef<'a, T> {
        const TAG_MASK: usize = 1;

        pub fn new(value: &'a T, tag: bool) -> Result<Self, InsufficientTagAlignment> {
            if align_of::<T>() < 2 {
                return Err(InsufficientTagAlignment);
            }

            let base = value as *const T;
            let tagged = base.map_addr(|address| {
                if tag {
                    address | Self::TAG_MASK
                } else {
                    address
                }
            });
            Ok(Self {
                tagged,
                owner: PhantomData,
            })
        }

        pub fn tag(&self) -> bool {
            self.tagged.addr() & Self::TAG_MASK != 0
        }

        pub fn get(&self) -> &'a T {
            let untagged = self.tagged.map_addr(|address| address & !Self::TAG_MASK);
            // SAFETY: `new` derives the pointer from `value`, preserves its
            // provenance and stores `PhantomData<&'a T>` to retain the borrow.
            unsafe { &*untagged }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum SwapError {
        OutOfBounds,
        SameIndex,
    }

    // SOLUTION: C45-E04
    pub fn swap_disjoint<T>(values: &mut [T], left: usize, right: usize) -> Result<(), SwapError> {
        if left >= values.len() || right >= values.len() {
            return Err(SwapError::OutOfBounds);
        }
        if left == right {
            return Err(SwapError::SameIndex);
        }

        let base = values.as_mut_ptr();
        // SAFETY: both indices are in the same live slice and are distinct;
        // the exclusive slice borrow prevents outside access during the swap.
        unsafe { ptr::swap(base.add(left), base.add(right)) };
        Ok(())
    }

    // SOLUTION: C45-E05
    pub struct LocalCounter {
        value: UnsafeCell<u64>,
    }

    impl LocalCounter {
        pub const fn new(value: u64) -> Self {
            Self {
                value: UnsafeCell::new(value),
            }
        }

        pub fn increment(&self) -> u64 {
            let pointer = self.value.get();
            // SAFETY: `UnsafeCell` permits mutation through a shared reference.
            // The type is not `Sync`, and this method performs no reentrant call.
            unsafe {
                *pointer += 1;
                *pointer
            }
        }

        pub fn get(&self) -> u64 {
            // SAFETY: this is a copy-only read, with no concurrent access because
            // `UnsafeCell` makes `LocalCounter` not `Sync`.
            unsafe { *self.value.get() }
        }
    }

    #[repr(C, packed)]
    pub struct PackedHeader {
        kind: u8,
        sequence: u32,
    }

    impl PackedHeader {
        pub const fn new(kind: u8, sequence: u32) -> Self {
            Self { kind, sequence }
        }

        pub const fn kind(&self) -> u8 {
            self.kind
        }

        // SOLUTION: C45-E06
        pub fn sequence(&self) -> u32 {
            let pointer = &raw const self.sequence;
            // SAFETY: the raw borrow avoids an intermediate reference and
            // `read_unaligned` accepts the packed field's reduced alignment.
            unsafe { pointer.read_unaligned() }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum MoveError {
        InvalidSource,
        InvalidDestination,
    }

    // SOLUTION: C45-E07
    pub fn copy_within_raw<T: Copy>(
        values: &mut [T],
        source: Range<usize>,
        destination_start: usize,
    ) -> Result<(), MoveError> {
        if source.start > source.end || source.end > values.len() {
            return Err(MoveError::InvalidSource);
        }
        let count = source.end - source.start;
        let destination_end = destination_start
            .checked_add(count)
            .ok_or(MoveError::InvalidDestination)?;
        if destination_end > values.len() {
            return Err(MoveError::InvalidDestination);
        }

        let base = values.as_mut_ptr();
        // SAFETY: both ranges are inside the same live slice; `ptr::copy`
        // explicitly permits overlap, and `T: Copy` makes both copies usable.
        unsafe { ptr::copy(base.add(source.start), base.add(destination_start), count) };
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn raw_read_checks_the_index_before_pointer_arithmetic() {
            let values = [10_u32, 20, 30];
            // SAFETY: pointer and length describe the same live initialized array.
            assert_eq!(
                unsafe { read_copy_at(values.as_ptr(), values.len(), 1) },
                Some(20)
            );
            // SAFETY: the out-of-range branch performs no pointer access.
            assert_eq!(unsafe { read_copy_at(ptr::null::<u32>(), 0, 0) }, None);
        }

        #[test]
        fn non_null_and_alignment_do_not_imply_dereferenceability() {
            let pointer = aligned_but_dangling::<u64>();
            assert_ne!(pointer.as_ptr().addr(), 0);
            assert_eq!(pointer.as_ptr().addr() % align_of::<u64>(), 0);
        }

        #[repr(align(2))]
        struct Word(u16);

        #[test]
        fn tagged_pointer_preserves_the_owner_and_provenance() {
            let value = Word(42);
            let tagged = TaggedRef::new(&value, true).unwrap();
            assert!(tagged.tag());
            assert_eq!(tagged.get().0, 42);
            assert!(TaggedRef::new(&7_u8, true).is_err());
        }

        #[test]
        fn raw_swap_uses_two_distinct_in_bounds_locations() {
            let mut values = ["left", "middle", "right"];
            assert_eq!(swap_disjoint(&mut values, 0, 2), Ok(()));
            assert_eq!(values, ["right", "middle", "left"]);
            assert_eq!(swap_disjoint(&mut values, 1, 1), Err(SwapError::SameIndex));
            assert_eq!(
                swap_disjoint(&mut values, 0, 3),
                Err(SwapError::OutOfBounds)
            );
        }

        #[test]
        fn unsafe_cell_supports_a_deliberately_single_threaded_api() {
            let counter = LocalCounter::new(40);
            assert_eq!(counter.increment(), 41);
            assert_eq!(counter.increment(), 42);
            assert_eq!(counter.get(), 42);
        }

        #[test]
        fn packed_field_is_read_without_creating_an_unaligned_reference() {
            let header = PackedHeader::new(3, 0x1020_3040);
            assert_eq!(header.kind(), 3);
            assert_eq!(header.sequence(), 0x1020_3040);
        }

        #[test]
        fn ptr_copy_handles_overlap_after_exact_range_checks() {
            let mut right = [1, 2, 3, 4, 5];
            assert_eq!(copy_within_raw(&mut right, 0..4, 1), Ok(()));
            assert_eq!(right, [1, 1, 2, 3, 4]);

            let mut left = [1, 2, 3, 4, 5];
            assert_eq!(copy_within_raw(&mut left, 1..5, 0), Ok(()));
            assert_eq!(left, [2, 3, 4, 5, 5]);
            let reversed = Range {
                start: left.len(),
                end: left.len() - 1,
            };
            assert_eq!(
                copy_within_raw(&mut left, reversed, 0),
                Err(MoveError::InvalidSource)
            );
            assert_eq!(
                copy_within_raw(&mut left, 0..3, 4),
                Err(MoveError::InvalidDestination)
            );
        }
    }
}

pub mod c46 {
    use std::mem::{align_of, offset_of, size_of};
    use std::ptr::NonNull;

    use thiserror::Error;

    #[repr(C)]
    struct DeclaredOrder {
        kind: u8,
        payload_length: u32,
        version: u16,
    }

    #[repr(C)]
    struct AlignmentOrder {
        payload_length: u32,
        version: u16,
        kind: u8,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct LayoutReport {
        pub size: usize,
        pub alignment: usize,
        pub kind_offset: usize,
        pub payload_length_offset: usize,
        pub version_offset: usize,
        pub padding_bytes: usize,
    }

    // SOLUTION: C46-E01
    pub fn compare_c_layouts() -> (LayoutReport, LayoutReport) {
        const PAYLOAD_BYTES: usize = size_of::<u8>() + size_of::<u32>() + size_of::<u16>();

        let declared = LayoutReport {
            size: size_of::<DeclaredOrder>(),
            alignment: align_of::<DeclaredOrder>(),
            kind_offset: offset_of!(DeclaredOrder, kind),
            payload_length_offset: offset_of!(DeclaredOrder, payload_length),
            version_offset: offset_of!(DeclaredOrder, version),
            padding_bytes: size_of::<DeclaredOrder>() - PAYLOAD_BYTES,
        };
        let alignment_order = LayoutReport {
            size: size_of::<AlignmentOrder>(),
            alignment: align_of::<AlignmentOrder>(),
            kind_offset: offset_of!(AlignmentOrder, kind),
            payload_length_offset: offset_of!(AlignmentOrder, payload_length),
            version_offset: offset_of!(AlignmentOrder, version),
            padding_bytes: size_of::<AlignmentOrder>() - PAYLOAD_BYTES,
        };
        (declared, alignment_order)
    }

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    #[error("porcentaje fuera de 0..=100: {0}")]
    pub struct InvalidPercentage(pub u8);

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(transparent)]
    /// A validated percentage whose public representation matches `u8`.
    ///
    /// The private field preserves the `0..=100` domain invariant; callers
    /// must still use `new` instead of manufacturing arbitrary bytes.
    pub struct Percentage(u8);

    // SOLUTION: C46-E02
    impl Percentage {
        pub fn new(value: u8) -> Result<Self, InvalidPercentage> {
            if value <= 100 {
                Ok(Self(value))
            } else {
                Err(InvalidPercentage(value))
            }
        }

        pub const fn get(self) -> u8 {
            self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    #[repr(u8)]
    pub enum RecordTag {
        User = 1,
        Order = 2,
    }

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    #[error("tag desconocido {0}")]
    pub struct UnknownTag(pub u8);

    // SOLUTION: C46-E03
    impl TryFrom<u8> for RecordTag {
        type Error = UnknownTag;

        fn try_from(value: u8) -> Result<Self, Self::Error> {
            match value {
                1 => Ok(Self::User),
                2 => Ok(Self::Order),
                other => Err(UnknownTag(other)),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct RecordHeader {
        pub version: u16,
        pub tag: RecordTag,
        pub payload_length: u32,
    }

    #[derive(Clone, Copy, Debug, Error, Eq, PartialEq)]
    pub enum DecodeError {
        #[error("cabecera incompleta")]
        Truncated,
        #[error("magic inválido {0:02x?}")]
        BadMagic([u8; 2]),
        #[error(transparent)]
        UnknownTag(#[from] UnknownTag),
    }

    // SOLUTION: C46-E05
    impl RecordHeader {
        pub const MAGIC: [u8; 2] = *b"DR";
        pub const ENCODED_LENGTH: usize = 9;

        pub fn encode(self) -> [u8; Self::ENCODED_LENGTH] {
            let mut bytes = [0; Self::ENCODED_LENGTH];
            bytes[0..2].copy_from_slice(&Self::MAGIC);
            bytes[2..4].copy_from_slice(&self.version.to_le_bytes());
            bytes[4] = self.tag as u8;
            bytes[5..9].copy_from_slice(&self.payload_length.to_le_bytes());
            bytes
        }

        pub fn decode(bytes: &[u8]) -> Result<Self, DecodeError> {
            let header = bytes
                .get(..Self::ENCODED_LENGTH)
                .ok_or(DecodeError::Truncated)?;
            let magic = [header[0], header[1]];
            if magic != Self::MAGIC {
                return Err(DecodeError::BadMagic(magic));
            }
            Ok(Self {
                version: u16::from_le_bytes([header[2], header[3]]),
                tag: RecordTag::try_from(header[4])?,
                payload_length: u32::from_le_bytes([header[5], header[6], header[7], header[8]]),
            })
        }
    }

    #[repr(C, packed)]
    pub struct PackedLength {
        kind: u8,
        payload_length: u32,
    }

    impl PackedLength {
        pub const fn new(kind: u8, payload_length: u32) -> Self {
            Self {
                kind,
                payload_length,
            }
        }

        pub const fn kind(&self) -> u8 {
            self.kind
        }

        // SOLUTION: C46-E04
        pub fn payload_length(&self) -> u32 {
            let pointer = &raw const self.payload_length;
            // SAFETY: the field is initialized inside this live value and
            // `read_unaligned` accepts its potentially reduced alignment.
            unsafe { pointer.read_unaligned() }
        }
    }

    // SOLUTION: C46-E06
    #[repr(align(64))]
    pub struct CacheLine<T>(T);

    impl<T> CacheLine<T> {
        pub const fn new(value: T) -> Self {
            Self(value)
        }

        pub const fn get(&self) -> &T {
            &self.0
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct NicheLayout {
        pub non_null_size: usize,
        pub option_size: usize,
        pub non_null_alignment: usize,
        pub option_alignment: usize,
    }

    // SOLUTION: C46-E07
    pub fn documented_non_null_niche<T>() -> NicheLayout {
        NicheLayout {
            non_null_size: size_of::<NonNull<T>>(),
            option_size: size_of::<Option<NonNull<T>>>(),
            non_null_alignment: align_of::<NonNull<T>>(),
            option_alignment: align_of::<Option<NonNull<T>>>(),
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn repr_c_measurements_distinguish_payload_from_padding() {
            let (declared, alignment_order) = compare_c_layouts();

            assert_eq!(declared.kind_offset, 0);
            assert!(declared.kind_offset < declared.payload_length_offset);
            assert!(declared.payload_length_offset < declared.version_offset);
            assert_eq!(alignment_order.payload_length_offset, 0);
            assert!(alignment_order.payload_length_offset < alignment_order.version_offset);
            assert!(alignment_order.version_offset < alignment_order.kind_offset);
            assert_eq!(declared.size % declared.alignment, 0);
            assert_eq!(alignment_order.size % alignment_order.alignment, 0);
            assert!(alignment_order.padding_bytes <= declared.padding_bytes);
        }

        #[test]
        fn transparent_wrapper_preserves_layout_but_validates_values() {
            assert_eq!(size_of::<Percentage>(), size_of::<u8>());
            assert_eq!(align_of::<Percentage>(), align_of::<u8>());
            assert_eq!(Percentage::new(75).map(Percentage::get), Ok(75));
            assert_eq!(Percentage::new(101), Err(InvalidPercentage(101)));
        }

        #[test]
        fn unknown_integer_never_becomes_an_invalid_enum() {
            assert_eq!(RecordTag::try_from(9), Err(UnknownTag(9)));
        }

        #[test]
        fn packed_field_is_accessed_without_an_unaligned_reference() {
            let packed = PackedLength::new(3, 0x1020_3040);
            assert_eq!(packed.kind(), 3);
            assert_eq!(packed.payload_length(), 0x1020_3040);
        }

        #[test]
        fn binary_format_is_explicit_and_round_trips() {
            let header = RecordHeader {
                version: 3,
                tag: RecordTag::Order,
                payload_length: 65_537,
            };
            let bytes = header.encode();
            assert_eq!(bytes, [b'D', b'R', 3, 0, 2, 1, 0, 1, 0]);
            assert_eq!(RecordHeader::decode(&bytes), Ok(header));
            assert_eq!(RecordHeader::decode(b"short"), Err(DecodeError::Truncated));

            let mut bad_magic = bytes;
            bad_magic[0] = b'X';
            assert_eq!(
                RecordHeader::decode(&bad_magic),
                Err(DecodeError::BadMagic(*b"XR")),
            );

            let mut bad_tag = bytes;
            bad_tag[4] = 99;
            assert_eq!(
                RecordHeader::decode(&bad_tag),
                Err(DecodeError::UnknownTag(UnknownTag(99))),
            );
        }

        #[test]
        fn raised_alignment_also_rounds_the_array_stride() {
            assert_eq!(align_of::<CacheLine<u8>>(), 64);
            assert_eq!(size_of::<CacheLine<u8>>(), 64);

            let lines = [CacheLine::new(10_u8), CacheLine::new(20_u8)];
            let distance = (&raw const lines[1]).addr() - (&raw const lines[0]).addr();
            assert_eq!(distance, size_of::<CacheLine<u8>>());
            assert_eq!(*lines[1].get(), 20);
        }

        #[test]
        fn only_the_documented_non_null_niche_is_asserted() {
            let report = documented_non_null_niche::<u32>();
            assert_eq!(report.non_null_size, report.option_size);
            assert_eq!(report.non_null_alignment, report.option_alignment);
        }
    }
}

pub mod c47 {
    use std::ffi::{CStr, c_char, c_int};
    use std::marker::PhantomData;
    use std::panic::{UnwindSafe, catch_unwind};
    use std::ptr::NonNull;
    use std::rc::Rc;
    use std::str::Utf8Error;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    unsafe fn raw_strlen(pointer: *const c_char) -> usize {
        // SAFETY: delegated to this private function's caller contract.
        unsafe { CStr::from_ptr(pointer) }.to_bytes().len()
    }

    // SOLUTION: C47-E01
    pub fn c_strlen(value: &CStr) -> usize {
        // SAFETY: `value` provides a live, non-null, NUL-terminated region for
        // the duration of the call; the simulated C function retains nothing.
        unsafe { raw_strlen(value.as_ptr()) }
    }

    struct RawBuffer {
        bytes: Vec<u8>,
        drops: Arc<AtomicUsize>,
    }

    fn buffer_create(bytes: Vec<u8>, drops: Arc<AtomicUsize>, should_fail: bool) -> *mut RawBuffer {
        if should_fail {
            return std::ptr::null_mut();
        }
        Box::into_raw(Box::new(RawBuffer { bytes, drops }))
    }

    unsafe fn buffer_destroy(pointer: *mut RawBuffer) {
        // SAFETY: el wrapper llama exactamente una vez con el puntero de Box::into_raw.
        let raw = unsafe { Box::from_raw(pointer) };
        raw.drops.fetch_add(1, Ordering::SeqCst);
        drop(raw);
    }

    pub struct Buffer {
        raw: NonNull<RawBuffer>,
        _not_send_or_sync: PhantomData<Rc<()>>,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct BufferCreationFailed;

    // SOLUTION: C47-E02
    impl Buffer {
        pub fn new(bytes: Vec<u8>, drops: Arc<AtomicUsize>) -> Self {
            Self::try_new(bytes, drops, false)
                .expect("the infallible simulated constructor returned null")
        }

        pub fn try_new(
            bytes: Vec<u8>,
            drops: Arc<AtomicUsize>,
            should_fail: bool,
        ) -> Result<Self, BufferCreationFailed> {
            let raw = NonNull::new(buffer_create(bytes, drops, should_fail))
                .ok_or(BufferCreationFailed)?;
            Ok(Self {
                raw,
                _not_send_or_sync: PhantomData,
            })
        }

        pub fn as_slice(&self) -> &[u8] {
            // SAFETY: `self` posee el handle y evita destruirlo durante el préstamo.
            unsafe { &self.raw.as_ref().bytes }
        }
    }

    impl Drop for Buffer {
        fn drop(&mut self) {
            // SAFETY: `raw` procede de `buffer_create` y Buffer no es Clone.
            unsafe { buffer_destroy(self.raw.as_ptr()) }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum FfiStatus {
        Ready,
        Busy,
        Unknown(c_int),
    }

    // SOLUTION: C47-E03
    pub const fn decode_status(code: c_int) -> FfiStatus {
        match code {
            0 => FfiStatus::Ready,
            1 => FfiStatus::Busy,
            unknown => FfiStatus::Unknown(unknown),
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CallbackLifecycleError {
        NotAccepting,
        NoCallbackInFlight,
        StillRegistered,
        CallbacksInFlight(usize),
        AlreadyReleased,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub struct CallbackSnapshot {
        pub accepting: bool,
        pub in_flight: usize,
        pub released: bool,
    }

    pub struct CallbackLifecycle {
        accepting: bool,
        in_flight: usize,
        released: bool,
    }

    // SOLUTION: C47-E04
    impl CallbackLifecycle {
        pub const fn registered() -> Self {
            Self {
                accepting: true,
                in_flight: 0,
                released: false,
            }
        }

        pub fn callback_started(&mut self) -> Result<(), CallbackLifecycleError> {
            if !self.accepting || self.released {
                return Err(CallbackLifecycleError::NotAccepting);
            }
            self.in_flight = self
                .in_flight
                .checked_add(1)
                .expect("el contador de callbacks no cabe en usize");
            Ok(())
        }

        pub fn callback_finished(&mut self) -> Result<(), CallbackLifecycleError> {
            if self.in_flight == 0 {
                return Err(CallbackLifecycleError::NoCallbackInFlight);
            }
            self.in_flight -= 1;
            Ok(())
        }

        pub fn begin_unregister(&mut self) -> Result<(), CallbackLifecycleError> {
            if self.released {
                return Err(CallbackLifecycleError::AlreadyReleased);
            }
            self.accepting = false;
            Ok(())
        }

        pub fn release_context(&mut self) -> Result<(), CallbackLifecycleError> {
            if self.released {
                return Err(CallbackLifecycleError::AlreadyReleased);
            }
            if self.accepting {
                return Err(CallbackLifecycleError::StillRegistered);
            }
            if self.in_flight != 0 {
                return Err(CallbackLifecycleError::CallbacksInFlight(self.in_flight));
            }
            self.released = true;
            Ok(())
        }

        pub const fn snapshot(&self) -> CallbackSnapshot {
            CallbackSnapshot {
                accepting: self.accepting,
                in_flight: self.in_flight,
                released: self.released,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    pub enum CText<'a> {
        Utf8(&'a str),
        Bytes(&'a [u8]),
    }

    // SOLUTION: C47-E05
    pub fn decode_c_text(value: &CStr, require_utf8: bool) -> Result<CText<'_>, Utf8Error> {
        if require_utf8 {
            value.to_str().map(CText::Utf8)
        } else {
            Ok(CText::Bytes(value.to_bytes()))
        }
    }

    pub const FFI_SUM_OK: c_int = 0;
    pub const FFI_SUM_NULL_OUTPUT: c_int = -1;
    pub const FFI_SUM_NULL_INPUT: c_int = -2;
    pub const FFI_SUM_OVERFLOW: c_int = -3;

    /// Sums a foreign pointer-length pair into an output parameter.
    ///
    /// # Safety
    ///
    /// If `output` is non-null, it must be aligned and valid for a `u64` write
    /// for the duration of the call. When `length > 0`,
    /// `values` must describe one live allocation with `length` initialized
    /// `u32` elements, and it must not overlap `output` incompatibly.
    // SOLUTION: C47-E06
    pub unsafe extern "C" fn sum_u32_export(
        values: *const u32,
        length: usize,
        output: *mut u64,
    ) -> c_int {
        if output.is_null() {
            return FFI_SUM_NULL_OUTPUT;
        }

        let values = if length == 0 {
            &[]
        } else {
            if values.is_null() {
                return FFI_SUM_NULL_INPUT;
            }
            // SAFETY: the public contract supplies the non-null case's region,
            // initialization, lifetime and aliasing requirements.
            unsafe { std::slice::from_raw_parts(values, length) }
        };

        let Some(sum) = values
            .iter()
            .try_fold(0_u64, |sum, value| sum.checked_add(u64::from(*value)))
        else {
            return FFI_SUM_OVERFLOW;
        };

        // SAFETY: the public contract makes a non-null output writable and the
        // input was fully consumed before this store.
        unsafe { output.write(sum) };
        FFI_SUM_OK
    }

    pub const FFI_CALL_PANIC: c_int = -128;

    // SOLUTION: C47-E07
    pub fn ffi_panic_firewall<F>(operation: F) -> c_int
    where
        F: FnOnce() -> c_int + UnwindSafe,
    {
        match catch_unwind(operation) {
            Ok(code) => code,
            Err(payload) => {
                // A hostile custom panic payload could itself panic in `Drop`.
                // Leaking only on this exceptional path keeps the FFI firewall.
                std::mem::forget(payload);
                FFI_CALL_PANIC
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::ffi::CString;

        #[test]
        fn cstr_proves_termination_and_hides_the_raw_pointer() {
            let value = CString::new("Rust").unwrap();
            assert_eq!(c_strlen(&value), 4);
        }

        #[test]
        fn raii_handle_destroys_exactly_once() {
            let drops = Arc::new(AtomicUsize::new(0));
            {
                let buffer = Buffer::try_new(vec![1, 2, 3], Arc::clone(&drops), false).unwrap();
                assert_eq!(buffer.as_slice(), [1, 2, 3]);
            }
            assert_eq!(drops.load(Ordering::SeqCst), 1);

            assert!(Buffer::try_new(Vec::new(), Arc::clone(&drops), true).is_err());
            assert_eq!(drops.load(Ordering::SeqCst), 1);
        }

        #[test]
        fn unknown_status_codes_are_preserved_as_data() {
            assert_eq!(decode_status(0), FfiStatus::Ready);
            assert_eq!(decode_status(1), FfiStatus::Busy);
            assert_eq!(decode_status(91), FfiStatus::Unknown(91));
        }

        #[test]
        fn callback_context_is_released_only_after_unregister_and_drain() {
            let mut lifecycle = CallbackLifecycle::registered();
            lifecycle.callback_started().unwrap();
            lifecycle.begin_unregister().unwrap();
            assert_eq!(
                lifecycle.release_context(),
                Err(CallbackLifecycleError::CallbacksInFlight(1)),
            );
            assert_eq!(
                lifecycle.callback_started(),
                Err(CallbackLifecycleError::NotAccepting),
            );
            lifecycle.callback_finished().unwrap();
            lifecycle.release_context().unwrap();
            assert_eq!(
                lifecycle.snapshot(),
                CallbackSnapshot {
                    accepting: false,
                    in_flight: 0,
                    released: true,
                },
            );
        }

        #[test]
        fn c_text_keeps_bytes_unless_utf8_is_explicitly_required() {
            let utf8 = CString::new("Rust").unwrap();
            assert_eq!(decode_c_text(&utf8, true), Ok(CText::Utf8("Rust")));

            let non_utf8 = c"\xff";
            assert_eq!(decode_c_text(non_utf8, false), Ok(CText::Bytes(b"\xff")),);
            assert!(decode_c_text(non_utf8, true).is_err());
        }

        #[test]
        fn pointer_length_export_validates_sentinels_before_access() {
            let values = [10_u32, 20, 12];
            let mut output = 0_u64;
            // SAFETY: input and output are disjoint live initialized regions.
            assert_eq!(
                unsafe { sum_u32_export(values.as_ptr(), values.len(), &mut output) },
                FFI_SUM_OK,
            );
            assert_eq!(output, 42);

            // SAFETY: the function explicitly accepts null input for length zero.
            assert_eq!(
                unsafe { sum_u32_export(std::ptr::null(), 0, &mut output) },
                FFI_SUM_OK,
            );
            assert_eq!(output, 0);

            // SAFETY: both error branches return before dereferencing null.
            assert_eq!(
                unsafe { sum_u32_export(std::ptr::null(), 2, &mut output) },
                FFI_SUM_NULL_INPUT,
            );
            assert_eq!(
                unsafe { sum_u32_export(values.as_ptr(), values.len(), std::ptr::null_mut()) },
                FFI_SUM_NULL_OUTPUT,
            );
        }

        #[test]
        fn panic_firewall_maps_unwinds_to_a_stable_code() {
            assert_eq!(ffi_panic_firewall(|| 7), 7);
            assert_eq!(ffi_panic_firewall(|| panic!("boom")), FFI_CALL_PANIC);
        }
    }
}

pub mod c48 {
    use std::marker::PhantomData;
    use std::mem::{self, MaybeUninit};
    use std::ptr::NonNull;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BufferSnapshot {
        pub length: usize,
        pub capacity: usize,
        pub initialized_prefix: usize,
        pub element_size: usize,
        pub allocation_live: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum BufferInvariantError {
        LengthExceedsCapacity,
        InitializedPrefixDiffersFromLength,
        MissingAllocation,
        UnexpectedAllocation,
        LayoutTooLarge,
    }

    // SOLUTION: C48-E01
    pub fn audit_buffer(snapshot: BufferSnapshot) -> Result<(), Vec<BufferInvariantError>> {
        let mut errors = Vec::new();

        if snapshot.length > snapshot.capacity {
            errors.push(BufferInvariantError::LengthExceedsCapacity);
        }
        if snapshot.initialized_prefix != snapshot.length {
            errors.push(BufferInvariantError::InitializedPrefixDiffersFromLength);
        }

        let needs_allocation = snapshot.element_size != 0 && snapshot.capacity != 0;
        match (needs_allocation, snapshot.allocation_live) {
            (true, false) => errors.push(BufferInvariantError::MissingAllocation),
            (false, true) => errors.push(BufferInvariantError::UnexpectedAllocation),
            _ => {}
        }

        if snapshot
            .capacity
            .checked_mul(snapshot.element_size)
            .is_none_or(|bytes| bytes > isize::MAX as usize)
        {
            errors.push(BufferInvariantError::LayoutTooLarge);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    struct InitGuard<'a, T> {
        storage: &'a mut [MaybeUninit<T>],
        initialized: usize,
    }

    impl<T> Drop for InitGuard<'_, T> {
        fn drop(&mut self) {
            for value in &mut self.storage[..self.initialized] {
                // SAFETY: el contador solo avanza después de escribir un T válido.
                unsafe { value.assume_init_drop() }
            }
        }
    }

    // SOLUTION: C48-E02
    pub fn try_init_array<T, E, F, const LENGTH: usize>(mut initialize: F) -> Result<[T; LENGTH], E>
    where
        F: FnMut(usize) -> Result<T, E>,
    {
        let mut storage: [MaybeUninit<T>; LENGTH] = std::array::from_fn(|_| MaybeUninit::uninit());
        let mut guard = InitGuard {
            storage: &mut storage,
            initialized: 0,
        };

        for index in 0..LENGTH {
            let value = initialize(index)?;
            guard.storage[index].write(value);
            guard.initialized += 1;
        }

        let pointer = guard.storage.as_ptr().cast::<[T; LENGTH]>();
        mem::forget(guard);
        // SAFETY: las LENGTH posiciones se inicializaron y el guard ya no las destruye.
        Ok(unsafe { pointer.read() })
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MemoryTask {
        BuildBeforeValidity,
        SuppressAutomaticDrop,
        DeliberatelyLeak,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum MemoryTool {
        MaybeUninit,
        ManuallyDrop,
        Forget,
    }

    // SOLUTION: C48-E03
    pub const fn choose_memory_tool(task: MemoryTask) -> MemoryTool {
        match task {
            MemoryTask::BuildBeforeValidity => MemoryTool::MaybeUninit,
            MemoryTask::SuppressAutomaticDrop => MemoryTool::ManuallyDrop,
            MemoryTask::DeliberatelyLeak => MemoryTool::Forget,
        }
    }

    pub struct RawOwner<T> {
        pointer: NonNull<T>,
        _owns: PhantomData<T>,
    }

    impl<T> RawOwner<T> {
        pub fn new(value: T) -> Self {
            let pointer = NonNull::from(Box::leak(Box::new(value)));
            Self {
                pointer,
                _owns: PhantomData,
            }
        }

        pub fn get(&self) -> &T {
            // SAFETY: `new` creó el allocation, `self` conserva ownership y
            // solo `&self` permite obtener este préstamo compartido.
            unsafe { self.pointer.as_ref() }
        }

        pub fn get_mut(&mut self) -> &mut T {
            // SAFETY: `&mut self` impide otros accesos a través de la API.
            unsafe { self.pointer.as_mut() }
        }
    }

    impl<T> Drop for RawOwner<T> {
        fn drop(&mut self) {
            // SAFETY: el puntero procede de un único `Box::leak`; nunca se
            // reconstruye antes y `Drop` se ejecuta como máximo una vez.
            unsafe { drop(Box::from_raw(self.pointer.as_ptr())) }
        }
    }

    // SOLUTION: C48-E04
    // SAFETY: `RawOwner` posee un único T y mover el owner mueve la facultad
    // exclusiva de accederlo y destruirlo. No existe estado externo afín al hilo.
    unsafe impl<T: Send> Send for RawOwner<T> {}

    // SAFETY: desde `&RawOwner<T>` solo se obtiene `&T`; no hay mutación
    // interior. Por tanto compartir el owner requiere exactamente `T: Sync`.
    unsafe impl<T: Sync> Sync for RawOwner<T> {}

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum AuditBoundary {
        SafeApi,
        UnsafeApi,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PremiseState {
        Proven,
        DelegatedToCaller,
        Missing,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct AuditPremise {
        pub name: &'static str,
        pub state: PremiseState,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct AuditFailure {
        pub missing: Vec<&'static str>,
        pub delegated_from_safe_api: Vec<&'static str>,
    }

    // SOLUTION: C48-E05
    pub fn audit_unsafe_boundary(
        boundary: AuditBoundary,
        premises: &[AuditPremise],
    ) -> Result<(), AuditFailure> {
        let missing = premises
            .iter()
            .filter(|premise| premise.state == PremiseState::Missing)
            .map(|premise| premise.name)
            .collect::<Vec<_>>();
        let delegated_from_safe_api = premises
            .iter()
            .filter(|premise| {
                boundary == AuditBoundary::SafeApi
                    && premise.state == PremiseState::DelegatedToCaller
            })
            .map(|premise| premise.name)
            .collect::<Vec<_>>();

        if missing.is_empty() && delegated_from_safe_api.is_empty() {
            Ok(())
        } else {
            Err(AuditFailure {
                missing,
                delegated_from_safe_api,
            })
        }
    }

    pub struct ReadCursor<'a, T> {
        pointer: NonNull<T>,
        remaining: usize,
        _borrow: PhantomData<&'a T>,
    }

    impl<'a, T> ReadCursor<'a, T> {
        pub fn new(values: &'a [T]) -> Self {
            let pointer = values.first().map_or_else(NonNull::dangling, NonNull::from);
            Self {
                pointer,
                remaining: values.len(),
                _borrow: PhantomData,
            }
        }

        pub fn first(&self) -> Option<&T> {
            if self.remaining == 0 {
                None
            } else {
                // SAFETY: el constructor liga el cursor al slice vivo; una
                // longitud no nula implica que `pointer` señala su primer T.
                Some(unsafe { self.pointer.as_ref() })
            }
        }
    }

    // SOLUTION: C48-E06
    pub fn shorten_cursor_lifetime<'long: 'short, 'short, T>(
        cursor: ReadCursor<'long, T>,
    ) -> ReadCursor<'short, T> {
        cursor
    }

    struct LengthGuard<'a, T> {
        vector: &'a mut Vec<T>,
        original_length: usize,
        initialized: usize,
    }

    impl<T> Drop for LengthGuard<'_, T> {
        fn drop(&mut self) {
            // SAFETY: cada incremento de `initialized` ocurre después de
            // escribir un T válido en un slot reservado distinto.
            unsafe { self.vector.set_len(self.original_length + self.initialized) }
        }
    }

    // SOLUTION: C48-E07
    pub fn try_extend_prefix<T, E, F>(
        vector: &mut Vec<T>,
        amount: usize,
        mut initialize: F,
    ) -> Result<(), E>
    where
        F: FnMut(usize) -> Result<T, E>,
    {
        vector.reserve(amount);
        let original_length = vector.len();
        let mut guard = LengthGuard {
            vector,
            original_length,
            initialized: 0,
        };

        for offset in 0..amount {
            let value = initialize(offset)?;
            // SAFETY: `reserve` garantizó espacio hasta `original_length +
            // amount`; este slot aún no pertenece al prefijo inicializado.
            unsafe {
                guard
                    .vector
                    .as_mut_ptr()
                    .add(original_length + offset)
                    .write(value)
            };
            guard.initialized += 1;
        }

        drop(guard);
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cell::Cell;
        use std::panic::{AssertUnwindSafe, catch_unwind};
        use std::rc::Rc;

        struct DropSpy(Rc<Cell<usize>>);

        impl Drop for DropSpy {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        #[test]
        fn buffer_audit_covers_allocated_empty_and_zst_states() {
            assert_eq!(
                audit_buffer(BufferSnapshot {
                    length: 2,
                    capacity: 4,
                    initialized_prefix: 2,
                    element_size: size_of::<u32>(),
                    allocation_live: true,
                }),
                Ok(()),
            );
            assert_eq!(
                audit_buffer(BufferSnapshot {
                    length: 8,
                    capacity: usize::MAX,
                    initialized_prefix: 8,
                    element_size: 0,
                    allocation_live: false,
                }),
                Ok(()),
            );

            let errors = audit_buffer(BufferSnapshot {
                length: 3,
                capacity: 2,
                initialized_prefix: 1,
                element_size: usize::MAX,
                allocation_live: false,
            })
            .unwrap_err();
            assert!(errors.contains(&BufferInvariantError::LengthExceedsCapacity));
            assert!(errors.contains(&BufferInvariantError::InitializedPrefixDiffersFromLength));
            assert!(errors.contains(&BufferInvariantError::MissingAllocation));
            assert!(errors.contains(&BufferInvariantError::LayoutTooLarge));
        }

        #[test]
        fn complete_initialization_returns_an_array() {
            let result = try_init_array::<_, (), _, 4>(|index| Ok(index * 2)).unwrap();
            assert_eq!(result, [0, 2, 4, 6]);
        }

        #[test]
        fn error_drops_only_initialized_elements() {
            let drops = Rc::new(Cell::new(0));
            let result = try_init_array::<_, &'static str, _, 4>(|index| {
                if index == 2 {
                    Err("stop")
                } else {
                    Ok(DropSpy(Rc::clone(&drops)))
                }
            });
            assert!(result.is_err());
            assert_eq!(drops.get(), 2);
        }

        #[test]
        fn memory_tools_are_not_interchangeable() {
            assert_eq!(
                choose_memory_tool(MemoryTask::BuildBeforeValidity),
                MemoryTool::MaybeUninit,
            );
            assert_eq!(
                choose_memory_tool(MemoryTask::SuppressAutomaticDrop),
                MemoryTool::ManuallyDrop,
            );
            assert_eq!(
                choose_memory_tool(MemoryTask::DeliberatelyLeak),
                MemoryTool::Forget,
            );
        }

        #[test]
        fn raw_owner_preserves_unique_ownership_and_thread_bounds() {
            fn assert_send_sync<T: Send + Sync>() {}
            assert_send_sync::<RawOwner<String>>();

            let mut owner = RawOwner::new(String::from("Rust"));
            owner.get_mut().push_str("onomicon");
            assert_eq!(owner.get(), "Rustonomicon");
        }

        #[test]
        fn safe_boundaries_cannot_delegate_unsafe_premises() {
            let premises = [
                AuditPremise {
                    name: "alineación",
                    state: PremiseState::Proven,
                },
                AuditPremise {
                    name: "lifetime",
                    state: PremiseState::DelegatedToCaller,
                },
            ];

            assert!(audit_unsafe_boundary(AuditBoundary::UnsafeApi, &premises).is_ok());
            assert_eq!(
                audit_unsafe_boundary(AuditBoundary::SafeApi, &premises),
                Err(AuditFailure {
                    missing: Vec::new(),
                    delegated_from_safe_api: vec!["lifetime"],
                }),
            );
        }

        #[test]
        fn read_cursor_is_covariant_over_its_borrow() {
            fn shorten<'short>(
                cursor: ReadCursor<'static, &'static str>,
                _scope: &'short (),
            ) -> ReadCursor<'short, &'static str> {
                shorten_cursor_lifetime(cursor)
            }

            static VALUES: [&str; 1] = ["ready"];
            let scope = ();
            let cursor = shorten(ReadCursor::new(&VALUES), &scope);
            assert_eq!(cursor.first(), Some(&"ready"));
        }

        #[test]
        fn prefix_extension_commits_only_written_values_on_error_and_panic() {
            let mut values = vec![10];
            let result = try_extend_prefix(&mut values, 4, |index| {
                if index == 2 {
                    Err("stop")
                } else {
                    Ok(index as i32)
                }
            });
            assert_eq!(result, Err("stop"));
            assert_eq!(values, [10, 0, 1]);

            let panic = catch_unwind(AssertUnwindSafe(|| {
                let _ = try_extend_prefix::<_, (), _>(&mut values, 3, |index| {
                    if index == 1 {
                        panic!("stop")
                    }
                    Ok(20 + index as i32)
                });
            }));
            assert!(panic.is_err());
            assert_eq!(values, [10, 0, 1, 20]);
        }
    }
}
