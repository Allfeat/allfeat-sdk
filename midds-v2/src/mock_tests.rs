use allfeat_midds_v2_codegen::runtime_midds;

#[cfg(feature = "runtime")]
extern crate alloc;
#[cfg(feature = "runtime")]
use alloc::vec;

/// Mock types for testing all macro functionalities

// Test 1: Unit struct
#[runtime_midds]
pub struct UnitStruct;

// Test 2: Newtype with String transformation
#[runtime_midds]
pub struct StringNewtype(#[runtime_bound(64)] String);

// Test 3: Newtype with Vec transformation
#[runtime_midds]
pub struct VecNewtype(#[runtime_bound(32)] Vec<u32>);

// Test 4: Tuple struct with multiple fields
#[runtime_midds]
pub struct TupleStruct(
    #[runtime_bound(16)] String,
    u32,
    #[runtime_bound(8)] Vec<u8>,
);

// Test 5: Named struct with all transformation types
#[runtime_midds]
pub struct NamedStruct {
    #[runtime_bound(128)]
    pub name: String,

    #[runtime_bound(64)]
    pub tags: Vec<u32>,

    #[runtime_bound(32)]
    pub optional_name: Option<String>,

    #[runtime_bound(16)]
    pub optional_tags: Option<Vec<u16>>,

    pub id: u64,
    pub active: bool,
}

// Test 6: Simple enum without transformation
#[runtime_midds]
pub enum SimpleEnum {
    Variant1,
    Variant2,
    Variant3,
}

// Test 7: Enum with single field variants using variant-level bounds
#[runtime_midds]
pub enum SingleFieldEnum {
    #[runtime_bound(48)]
    Text(String),
    Number(u32),
    #[runtime_bound(24)]
    List(Vec<u8>),
}

// Test 8: Enum with multiple field variants using variant-level bounds
#[runtime_midds]
pub enum MultiFieldEnum {
    Simple,
    #[runtime_bound(32)]
    Pair(String, u32),
    #[runtime_bound(16)]
    Triple(String, u32, Vec<u8>),
}

// Test 9: Enum with named field variants (struct-like variants)
#[runtime_midds]
pub enum StructVariantEnum {
    Unit,
    Named {
        id: u32,
        active: bool,
        count: Option<u32>,
    },
}

// Test 10: Complex nested types
#[runtime_midds]
pub struct NestedStruct {
    #[runtime_bound(256)]
    pub deep_option: Option<Option<String>>,

    #[runtime_bound(128)]
    pub vec_of_options: Vec<Option<u32>>,

    #[runtime_bound(64)]
    pub option_of_vec: Option<Vec<u32>>,

    pub counter: u64,
}

// Test 11: Reference types (&str)
#[runtime_midds]
pub struct RefStruct {
    pub id: u32,
    // Note: &str fields would need lifetime parameters, which is complex for this test
    // We'll test this in the transform function unit tests instead
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(not(feature = "runtime"))]
    fn test_non_runtime_types() {
        // Test that without runtime feature, types remain as standard Rust types

        // Unit struct
        let _unit = UnitStruct;

        // Newtype structs
        let string_newtype = StringNewtype("test".to_string());
        assert_eq!(string_newtype.0, "test");

        let vec_newtype = VecNewtype(vec![1, 2, 3]);
        assert_eq!(vec_newtype.0, vec![1, 2, 3]);

        // Tuple struct
        let tuple = TupleStruct("hello".to_string(), 42, vec![1, 2]);
        assert_eq!(tuple.0, "hello");
        assert_eq!(tuple.1, 42);
        assert_eq!(tuple.2, vec![1, 2]);

        // Named struct
        let named = NamedStruct {
            name: "test".to_string(),
            tags: vec![1, 2],
            optional_name: Some("opt".to_string()),
            optional_tags: Some(vec![1, 2, 3]),
            id: 123,
            active: true,
        };
        assert_eq!(named.name, "test");
        assert_eq!(named.tags, vec![1, 2]);
        assert_eq!(named.optional_name, Some("opt".to_string()));
        assert_eq!(named.optional_tags, Some(vec![1, 2, 3]));

        // Simple enum - test all variants to avoid dead code warnings
        let simple1 = SimpleEnum::Variant1;
        let simple2 = SimpleEnum::Variant2;
        let simple3 = SimpleEnum::Variant3;
        assert!(matches!(simple1, SimpleEnum::Variant1));
        assert!(matches!(simple2, SimpleEnum::Variant2));
        assert!(matches!(simple3, SimpleEnum::Variant3));

        // Single field enum
        let single = SingleFieldEnum::Text("hello".to_string());
        if let SingleFieldEnum::Text(text) = single {
            assert_eq!(text, "hello");
        }

        // Multi field enum
        let multi = MultiFieldEnum::Pair("key".to_string(), 42);
        if let MultiFieldEnum::Pair(key, value) = multi {
            assert_eq!(key, "key");
            assert_eq!(value, 42);
        }

        // Struct variant enum - test both variants to avoid dead code warnings
        let struct_variant1 = StructVariantEnum::Unit;
        let struct_variant2 = StructVariantEnum::Named {
            id: 42,
            active: true,
            count: Some(10),
        };

        assert!(matches!(struct_variant1, StructVariantEnum::Unit));

        if let StructVariantEnum::Named { id, active, count } = struct_variant2 {
            assert_eq!(id, 42);
            assert_eq!(active, true);
            assert_eq!(count, Some(10));
        }

        // Nested struct
        let nested = NestedStruct {
            deep_option: Some(Some("deep".to_string())),
            vec_of_options: vec![Some(1), None, Some(2)],
            option_of_vec: Some(vec![1, 2]),
            counter: 999,
        };
        assert_eq!(nested.deep_option, Some(Some("deep".to_string())));
        assert_eq!(nested.vec_of_options.len(), 3);
        assert_eq!(nested.counter, 999);
    }

    #[test]
    #[cfg(feature = "runtime")]
    fn test_runtime_bounded_types() {
        use frame_support::{traits::ConstU32, BoundedVec};

        // Test BoundedVec conversions for String fields
        let string_bytes: BoundedVec<u8, ConstU32<64>> =
            BoundedVec::try_from("test".as_bytes().to_vec()).unwrap();
        let string_newtype = StringNewtype(string_bytes.clone());
        assert_eq!(string_newtype.0, string_bytes);

        // Test BoundedVec conversions for Vec fields
        let vec_bounded: BoundedVec<u32, ConstU32<32>> =
            BoundedVec::try_from(vec![1, 2, 3]).unwrap();
        let vec_newtype = VecNewtype(vec_bounded.clone());
        assert_eq!(vec_newtype.0, vec_bounded);

        // Test tuple struct with mixed types
        let tuple = TupleStruct(
            BoundedVec::try_from("hello".as_bytes().to_vec()).unwrap(),
            42,
            BoundedVec::try_from(vec![1, 2]).unwrap(),
        );
        assert_eq!(tuple.1, 42);

        // Test named struct with Option<BoundedVec>
        let named = NamedStruct {
            name: BoundedVec::try_from("test".as_bytes().to_vec()).unwrap(),
            tags: BoundedVec::try_from(vec![1, 2]).unwrap(),
            optional_name: Some(BoundedVec::try_from("opt".as_bytes().to_vec()).unwrap()),
            optional_tags: Some(BoundedVec::try_from(vec![1, 2, 3]).unwrap()),
            id: 123,
            active: true,
        };
        assert_eq!(named.id, 123);
        assert_eq!(named.active, true);

        // Test enum with BoundedVec
        let single =
            SingleFieldEnum::Text(BoundedVec::try_from("hello".as_bytes().to_vec()).unwrap());
        if let SingleFieldEnum::Text(text) = single {
            assert_eq!(text.as_slice(), "hello".as_bytes());
        }

        // Test bounds enforcement
        let too_long_string = "a".repeat(65); // 65 chars > 64 bound
        let result: Result<BoundedVec<u8, ConstU32<64>>, _> =
            BoundedVec::try_from(too_long_string.as_bytes().to_vec());
        assert!(result.is_err());
    }

    #[test]
    #[cfg(feature = "runtime")]
    fn test_runtime_derives_present() {
        // This test ensures that all the Substrate traits are derived
        // when the runtime feature is enabled

        use parity_scale_codec::{Decode, Encode};
        use scale_info::TypeInfo;

        // Test that types implement required traits
        let unit = UnitStruct;
        let _encoded = unit.encode();

        // Test TypeInfo is available
        assert!(!UnitStruct::type_info().path.segments.is_empty());
        assert!(!SimpleEnum::type_info().path.segments.is_empty());
        assert!(!NamedStruct::type_info().path.segments.is_empty());

        // Test that we can create and encode/decode a complex type
        use frame_support::BoundedVec;

        let original = NamedStruct {
            name: BoundedVec::try_from("test".as_bytes().to_vec()).unwrap(),
            tags: BoundedVec::try_from(vec![]).unwrap(),
            optional_name: None,
            optional_tags: None,
            id: 42,
            active: false,
        };

        let encoded = original.encode();
        let decoded: NamedStruct = Decode::decode(&mut &encoded[..]).unwrap();
        assert_eq!(decoded.id, 42);
        assert_eq!(decoded.active, false);
    }

    #[test]
    #[cfg(not(feature = "runtime"))]
    fn test_non_runtime_derives_only() {
        // Test that only basic derives are present without runtime feature
        let unit1 = UnitStruct;
        let unit2 = UnitStruct;

        // Debug, Clone, PartialEq, Eq should be available
        let _debug = format!("{:?}", unit1);
        let _cloned = unit1.clone();
        assert_eq!(unit1, unit2);

        // Test with more complex types
        let simple1 = SimpleEnum::Variant1;
        let simple2 = SimpleEnum::Variant1;
        let simple3 = SimpleEnum::Variant2;

        assert_eq!(simple1, simple2);
        assert_ne!(simple1, simple3);
        let _cloned_enum = simple1.clone();
    }

    #[test]
    fn test_nested_option_transformations() {
        // This test works for both runtime and non-runtime features
        // as the structure is the same, only the inner types change

        #[cfg(not(feature = "runtime"))]
        {
            let nested = NestedStruct {
                deep_option: Some(Some("nested".to_string())),
                vec_of_options: vec![Some(1), None],
                option_of_vec: Some(vec![1, 2]),
                counter: 1,
            };

            assert!(nested.deep_option.is_some());
            assert_eq!(nested.vec_of_options.len(), 2);
        }

        #[cfg(feature = "runtime")]
        {
            use frame_support::BoundedVec;

            let nested = NestedStruct {
                deep_option: Some(Some(
                    BoundedVec::try_from("nested".as_bytes().to_vec()).unwrap(),
                )),
                vec_of_options: BoundedVec::try_from(vec![Some(1), None]).unwrap(),
                option_of_vec: Some(BoundedVec::try_from(vec![1, 2]).unwrap()),
                counter: 1,
            };

            assert!(nested.deep_option.is_some());
            assert_eq!(nested.counter, 1);
        }
    }
}

// Test for type transformation function coverage
#[cfg(test)]
mod transform_tests {
    use super::*;

    #[test]
    fn test_all_transformation_paths() {
        // These are compile-time tests - if they compile, the transformations work

        // Test String -> BoundedVec<u8, ConstU32<N>>
        #[runtime_midds]
        struct StringTest {
            #[runtime_bound(10)]
            field: String,
        }

        // Test Vec<T> -> BoundedVec<T, ConstU32<N>>
        #[runtime_midds]
        struct VecTest {
            #[runtime_bound(10)]
            field: Vec<u32>,
        }

        // Test Option<String> -> Option<BoundedVec<u8, ConstU32<N>>>
        #[runtime_midds]
        struct OptionStringTest {
            #[runtime_bound(10)]
            field: Option<String>,
        }

        // Test Option<Vec<T>> -> Option<BoundedVec<T, ConstU32<N>>>
        #[runtime_midds]
        struct OptionVecTest {
            #[runtime_bound(10)]
            field: Option<Vec<u32>>,
        }

        // Test nested Option<Option<String>>
        #[runtime_midds]
        struct NestedOptionTest {
            #[runtime_bound(10)]
            field: Option<Option<String>>,
        }

        // Test that untransformed types remain unchanged
        #[runtime_midds]
        struct UnchangedTest {
            number: u32,
            flag: bool,
            optional_number: Option<u32>,
        }

        // If we reach here, all transformations compiled successfully
        assert!(true);
    }
}
