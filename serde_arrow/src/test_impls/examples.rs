use super::macros::*;

test_example!(
    test_name = benchmark_primitives,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new("a", GenericDataType::U8, false))
        .with_child(GenericField::new("b", GenericDataType::U16, false))
        .with_child(GenericField::new("c", GenericDataType::U32, false))
        .with_child(GenericField::new("d", GenericDataType::U64, false))
        .with_child(GenericField::new("e", GenericDataType::I8, false))
        .with_child(GenericField::new("f", GenericDataType::I16, false))
        .with_child(GenericField::new("g", GenericDataType::I32, false))
        .with_child(GenericField::new("h", GenericDataType::I64, false))
        .with_child(GenericField::new("i", GenericDataType::F32, false))
        .with_child(GenericField::new("j", GenericDataType::F64, false))
        .with_child(GenericField::new("k", GenericDataType::Bool, false)),
    ty = Item,
    values = [Item::default(), Item::default()],
    nulls = [false, false],
    define = {
        #[derive(Default, Serialize, Deserialize, Debug, PartialEq)]
        struct Item {
            pub a: u8,
            pub b: u16,
            pub c: u32,
            pub d: u64,
            pub e: i8,
            pub f: i16,
            pub g: i32,
            pub h: i64,
            pub i: f32,
            pub j: f64,
            pub k: bool,
        }
    },
);

test_example!(
    test_name = benchmark_complex_1,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new(
            "string",
            GenericDataType::LargeUtf8,
            false
        ))
        .with_child(
            GenericField::new("points", GenericDataType::LargeList, false).with_child(
                GenericField::new("element", GenericDataType::Struct, false)
                    .with_child(GenericField::new("x", GenericDataType::F32, false))
                    .with_child(GenericField::new("y", GenericDataType::F32, false))
            )
        )
        .with_child(
            GenericField::new("float", GenericDataType::Union, false)
                .with_child(GenericField::new("F32", GenericDataType::F32, false))
                .with_child(GenericField::new("F64", GenericDataType::F64, false))
        ),
    ty = Item,
    values = [
        Item {
            string: "foo".into(),
            points: vec![Point { x: 0.0, y: 0.0 }],
            float: Float::F32(13.0)
        },
        Item {
            string: "foo".into(),
            points: vec![],
            float: Float::F64(21.0)
        },
    ],
    nulls = [false, false],
    define = {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Item {
            string: String,
            points: Vec<Point>,
            float: Float,
        }

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        enum Float {
            F32(f32),
            F64(f64),
        }

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Point {
            x: f32,
            y: f32,
        }
    },
);

test_example!(
    test_name = benchmark_complex_2,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new(
            "string",
            GenericDataType::LargeUtf8,
            false
        ))
        .with_child(
            GenericField::new("points", GenericDataType::LargeList, false).with_child(
                GenericField::new("element", GenericDataType::Struct, false)
                    .with_child(GenericField::new("x", GenericDataType::F32, false))
                    .with_child(GenericField::new("y", GenericDataType::F32, false))
            )
        )
        .with_child(
            GenericField::new("child", GenericDataType::Struct, false)
                .with_child(GenericField::new("a", GenericDataType::Bool, false))
                .with_child(GenericField::new("b", GenericDataType::F64, false))
                .with_child(GenericField::new("c", GenericDataType::F32, true))
        ),
    ty = Item,
    values = [
        Item {
            string: "foo".into(),
            points: vec![Point { x: 0.0, y: 1.0 }, Point { x: 2.0, y: 3.0 },],
            child: SubItem {
                a: true,
                b: 42.0,
                c: None,
            },
        },
        Item {
            string: "bar".into(),
            points: vec![],
            child: SubItem {
                a: false,
                b: 13.0,
                c: Some(7.0),
            },
        },
    ],
    nulls = [false, false],
    define = {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Item {
            string: String,
            points: Vec<Point>,
            child: SubItem,
        }

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Point {
            x: f32,
            y: f32,
        }

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct SubItem {
            a: bool,
            b: f64,
            c: Option<f32>,
        }
    },
);

test_example!(
    test_name = nested_options,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new("a", GenericDataType::U8, false))
        .with_child(GenericField::new("b", GenericDataType::U16, true))
        .with_child(GenericField::new("c", GenericDataType::U32, true)),
    ty = Item,
    values = [
        Item {
            a: 0,
            b: Some(1),
            c: Some(Some(2)),
        },
        Item {
            a: 0,
            b: None,
            c: Some(None),
        },
        Item {
            a: 0,
            b: None,
            c: None,
        },
    ],
    expected_values = [
        Item {
            a: 0,
            b: Some(1),
            c: Some(Some(2)),
        },
        Item {
            a: 0,
            b: None,
            // NOTE: the arrow format only has a single level of "nullness"
            // therefore `None` and `Some(None)` cannot be distinguished
            c: None,
        },
        Item {
            a: 0,
            b: None,
            c: None,
        },
    ],
    nulls = [false, false, false],
    define = {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct Item {
            a: u8,
            b: Option<u16>,
            c: Option<Option<u32>>,
        }
    },
);

test_example!(
    test_name = fieldless_unions_in_a_struct,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new("foo", GenericDataType::U32, false))
        .with_child(
            GenericField::new("bar", GenericDataType::Union, false)
                .with_child(GenericField::new("A", GenericDataType::Null, true))
                .with_child(GenericField::new("B", GenericDataType::Null, true))
                .with_child(GenericField::new("C", GenericDataType::Null, true))
        )
        .with_child(GenericField::new("baz", GenericDataType::F32, false)),
    ty = S,
    values = [
        S {
            foo: 0,
            bar: U::A,
            baz: 1.0,
        },
        S {
            foo: 2,
            bar: U::B,
            baz: 3.0,
        },
        S {
            foo: 4,
            bar: U::C,
            baz: 5.0,
        },
        S {
            foo: 6,
            bar: U::A,
            baz: 7.0,
        },
    ],
    nulls = [false, false, false, false],
    define = {
        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        struct S {
            foo: u32,
            bar: U,
            baz: f32,
        }

        #[derive(Serialize, Deserialize, Debug, PartialEq)]
        enum U {
            A,
            B,
            C,
        }
    },
);

test_example!(
    // see https://github.com/chmp/serde_arrow/issues/57
    test_name = issue_57,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new(
            "filename",
            GenericDataType::LargeUtf8,
            false
        ))
        .with_child(
            GenericField::new("game_type", GenericDataType::Union, false)
                .with_child(
                    GenericField::new("", GenericDataType::Null, true)
                        .with_strategy(Strategy::UnknownVariant)
                )
                .with_child(GenericField::new(
                    "RegularSeason",
                    GenericDataType::Null,
                    true
                ))
        )
        .with_child(
            GenericField::new("account_type", GenericDataType::Union, false)
                .with_child(
                    GenericField::new("", GenericDataType::Null, true)
                        .with_strategy(Strategy::UnknownVariant)
                )
                .with_child(GenericField::new("Deduced", GenericDataType::Null, true))
        )
        .with_child(GenericField::new("file_index", GenericDataType::U64, false)),
    ty = FileInfo,
    values = [FileInfo {
        filename: String::from("test"),
        game_type: GameType::RegularSeason,
        account_type: AccountType::Deduced,
        file_index: 0
    },],
    nulls = [false],
    define = {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        pub enum AccountType {
            PlayByPlay,
            Deduced,
            BoxScore,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        pub enum GameType {
            SpringTraining,
            RegularSeason,
            AllStarGame,
            WildCardSeries,
            DivisionSeries,
            LeagueChampionshipSeries,
            WorldSeries,
            NegroLeagues,
            Other,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        pub struct FileInfo {
            pub filename: String,
            pub game_type: GameType,
            pub account_type: AccountType,
            pub file_index: usize,
        }
    },
);

test_roundtrip_arrays!(
    simple_example {
        #[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
        struct S {
            a: f32,
            b: u32,
        }

        let items = &[
            S{ a: 2.0, b: 4 },
            S{ a: -123.0, b: 9 },
        ];
        let fields = &[
            GenericField::new("a", GenericDataType::F32, false),
            GenericField::new("b", GenericDataType::U32, false),
        ];
    }
    assert_round_trip(fields, items);
);

test_roundtrip_arrays!(
    toplevel_nullables {
        #[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
        struct S {
            a: Option<f32>,
            b: Option<u32>,
        }

        let items = &[
            S{ a: Some(2.0), b: None },
            S{ a: None, b: Some(9) },
        ];
        let fields = &[
            GenericField::new("a", GenericDataType::F32, true),
            GenericField::new("b", GenericDataType::U32, true),
        ];
    }
    assert_round_trip(fields, items);
);

test_example!(
    test_name = nullable_vec_bool,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::LargeList, true)
        .with_child(GenericField::new("element", GenericDataType::Bool, false)),
    ty = Option<Vec<bool>>,
    values = [Some(vec![true, false]), None, Some(vec![])],
);

test_example!(
    test_name = nullable_vec_bool_nested,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::LargeList, true)
        .with_child(GenericField::new("element", GenericDataType::LargeList, false)
            .with_child(GenericField::new("element", GenericDataType::Bool, false))),
    ty = Option<Vec<Vec<bool>>>,
    values = [Some(vec![vec![true], vec![false, false]]), None, Some(vec![vec![]])],
);

test_example!(
    test_name = vec_nullable_bool,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::LargeList, false)
        .with_child(GenericField::new("element", GenericDataType::Bool, true)),
    ty = Vec<Option<bool>>,
    values = [vec![Some(true), Some(false)], vec![], vec![None, Some(false)]],
);

test_example!(
    test_name = struct_nullable,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("root",GenericDataType::Struct, true)
        .with_child(GenericField::new("a", GenericDataType::Bool, false))
        .with_child(GenericField::new("b", GenericDataType::I64, false))
        .with_child(GenericField::new("c", GenericDataType::Null, true))
        .with_child(GenericField::new("d", GenericDataType::LargeUtf8, false)),
    ty = Option<Struct>,
    values = [
        Some(Struct {
            a: true,
            b: 42,
            c: (),
            d: String::from("hello"),
        }),
        None,
        Some(Struct {
            a: false,
            b: 13,
            c: (),
            d: String::from("world"),
        }),
    ],
    define = {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Outer {
            inner: Struct,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Struct {
            a: bool,
            b: i64,
            c: (),
            d: String,
        }
    },
);

test_example!(
    test_name = struct_nullable_nested,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("root",GenericDataType::Struct, true)
        .with_child(GenericField::new("inner", GenericDataType::Struct, false)
            .with_child(GenericField::new("a", GenericDataType::Bool, false))
            .with_child(GenericField::new("b", GenericDataType::I64, false))
            .with_child(GenericField::new("c", GenericDataType::Null, true))
            .with_child(GenericField::new("d", GenericDataType::LargeUtf8, false))),
    ty = Option<Outer>,
    values = [
        Some(Outer {
            inner: Struct {
            a: true,
            b: 42,
            c: (),
            d: String::from("hello"),
        }}),
        None,
        Some(Outer {inner: Struct {
            a: false,
            b: 13,
            c: (),
            d: String::from("world"),
        }}),
    ],
    define = {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Outer {
            inner: Struct,
        }

        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct Struct {
            a: bool,
            b: i64,
            c: (),
            d: String,
        }
    },
);

test_example!(
    test_name = struct_nullable_item,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("root", GenericDataType::Struct, false)
        .with_child(GenericField::new("a", GenericDataType::Bool, true))
        .with_child(GenericField::new("b", GenericDataType::I64, true))
        .with_child(GenericField::new("c", GenericDataType::Null, true))
        .with_child(GenericField::new("d", GenericDataType::LargeUtf8, true)),
    ty = StructNullable,
    values = [
        StructNullable {
            a: None,
            b: None,
            c: None,
            d: Some(String::from("hello")),
        },
        StructNullable {
            a: Some(true),
            b: Some(42),
            c: None,
            d: None,
        },
    ],
    define = {
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        struct StructNullable {
            a: Option<bool>,
            b: Option<i64>,
            c: Option<()>,
            d: Option<String>,
        }
    },
);

test_example!(
    test_name = tuple_nullable,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Struct, true)
        .with_strategy(Strategy::TupleAsStruct)
        .with_child(GenericField::new("0", GenericDataType::Bool, false))
        .with_child(GenericField::new("1", GenericDataType::I64, false)),
    ty = Option<(bool, i64)>,
    values = [
        Some((true, 21)),
        None,
        Some((false, 42)),
    ],
);

test_example!(
    test_name = tuple_nullable_nested,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Struct, true)
        .with_strategy(Strategy::TupleAsStruct)
        .with_child(GenericField::new("0", GenericDataType::Struct, false)
            .with_strategy(Strategy::TupleAsStruct)
            .with_child(GenericField::new("0", GenericDataType::Bool, false))
            .with_child(GenericField::new("1", GenericDataType::I64, false)))
        .with_child(GenericField::new("1", GenericDataType::I64, false)),
    ty = Option<((bool, i64), i64)>,
    values = [
        Some(((true, 21), 7)),
        None,
        Some(((false, 42), 13)),
    ],
);

test_example!(
    test_name = enums,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Union, false)
        .with_child(GenericField::new("U8", GenericDataType::U8, false))
        .with_child(GenericField::new("U16", GenericDataType::U16, false))
        .with_child(GenericField::new("U32", GenericDataType::U32, false))
        .with_child(GenericField::new("U64", GenericDataType::U64, false)),
    ty = Item,
    values = [Item::U32(2), Item::U64(3), Item::U8(0), Item::U16(1),],
    define = {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        enum Item {
            U8(u8),
            U16(u16),
            U32(u32),
            U64(u64),
        }
    },
);

test_example!(
    test_name = enums_tuple,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Union, false)
        .with_child(
            GenericField::new("A", GenericDataType::Struct, false)
                .with_strategy(Strategy::TupleAsStruct)
                .with_child(GenericField::new("0", GenericDataType::U8, false))
                .with_child(GenericField::new("1", GenericDataType::U32, false))
        )
        .with_child(
            GenericField::new("B", GenericDataType::Struct, false)
                .with_strategy(Strategy::TupleAsStruct)
                .with_child(GenericField::new("0", GenericDataType::U16, false))
                .with_child(GenericField::new("1", GenericDataType::U64, false))
        ),
    ty = Item,
    values = [Item::A(2, 3), Item::B(0, 1),],
    define = {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        enum Item {
            A(u8, u32),
            B(u16, u64),
        }
    },
);

test_example!(
    test_name = enums_struct,
    test_bytecode_deserialization = true,
    field = GenericField::new("root", GenericDataType::Union, false)
        .with_child(
            GenericField::new("A", GenericDataType::Struct, false)
                .with_child(GenericField::new("a", GenericDataType::U8, false))
                .with_child(GenericField::new("b", GenericDataType::U32, false))
        )
        .with_child(
            GenericField::new("B", GenericDataType::Struct, false)
                .with_child(GenericField::new("c", GenericDataType::U16, false))
                .with_child(GenericField::new("d", GenericDataType::U64, false))
        ),
    ty = Item,
    values = [Item::A { a: 2, b: 3 }, Item::B { c: 0, d: 1 },],
    define = {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        enum Item {
            A { a: u8, b: u32 },
            B { c: u16, d: u64 },
        }
    },
);

test_example!(
    test_name = enums_union,
    test_bytecode_deserialization = true,
    tracing_options = TracingOptions::default().allow_null_fields(true),
    field = GenericField::new("root", GenericDataType::Union, false)
        .with_child(GenericField::new("A", GenericDataType::Null, true))
        .with_child(GenericField::new("B", GenericDataType::Null, true)),
    ty = Item,
    values = [Item::A, Item::B,],
    define = {
        #[derive(Debug, PartialEq, Deserialize, Serialize)]
        enum Item {
            A,
            B,
        }
    },
);

// TODO: fix more examples
/*
test_example!(
    test_name = hash_maps,
    tracing_options = TracingOptions::new().map_as_struct(false),
    field = Field::new(
        "value",
        DataType::Map(
            Box::new(Field::new(
                "entries",
                DataType::Struct(vec![
                    Field::new("key", GenericDataType::I64, false),
                    Field::new("value", GenericDataType::Bool, false),
                ]),
                false
            )),
            false,
        ),
        false,
    ),
    ty = HashMap<i64, bool>,
    values = [
        hashmap!{0 => true, 1 => false, 2 => true},
        hashmap!{3 => false, 4 => true},
        hashmap!{},
    ],
);

test_example!(
    test_name = hash_maps_nullable,
    tracing_options = TracingOptions::new().map_as_struct(false),
    field = Field::new(
        "value",
        DataType::Map(
            Box::new(Field::new(
                "entries",
                DataType::Struct(vec![
                    Field::new("key", GenericDataType::I64, false),
                    Field::new("value", GenericDataType::Bool, false),
                ]),
                false
            )),
            false,
        ),
        true,
    ),
    ty = Option<HashMap<i64, bool>>,
    values = [
        Some(hashmap!{0 => true, 1 => false, 2 => true}),
        Some(hashmap!{3 => false, 4 => true}),
        Some(hashmap!{}),
    ],
);

test_example!(
    test_name = hash_maps_nullable_keys,
    tracing_options = TracingOptions::new().map_as_struct(false),
    field = Field::new(
        "value",
        DataType::Map(
            Box::new(Field::new(
                "entries",
                DataType::Struct(vec![
                    Field::new("key", GenericDataType::I64, true),
                    Field::new("value", GenericDataType::Bool, false),
                ]),
                false
            )),
            false,
        ),
        false,
    ),
    ty = HashMap<Option<i64>, bool>,
    values = [
        hashmap!{Some(0) => true, Some(1) => false, Some(2) => true},
        hashmap!{Some(3) => false, Some(4) => true},
        hashmap!{},
    ],
);

test_example!(
    test_name = hash_maps_nullable_values,
    tracing_options = TracingOptions::new().map_as_struct(false),
    field = Field::new(
        "value",
        DataType::Map(
            Box::new(Field::new(
                "entries",
                DataType::Struct(vec![
                    Field::new("key", GenericDataType::I64, false),
                    Field::new("value", GenericDataType::Bool, true),
                ]),
                false
            )),
            false,
        ),
        false,
    ),
    ty = HashMap<i64, Option<bool>>,
    values = [
        hashmap!{0 => Some(true), 1 => Some(false), 2 => Some(true)},
        hashmap!{3 => Some(false), 4 => Some(true)},
        hashmap!{},
    ],
);

test_example!(
    test_name = btree_maps,
    tracing_options = TracingOptions::new().map_as_struct(false),
    field = Field::new(
        "value",
        DataType::Map(
            Box::new(Field::new(
                "entries",
                DataType::Struct(vec![
                    Field::new("key", GenericDataType::I64, false),
                    Field::new("value", GenericDataType::Bool, false),
                ]),
                false
            )),
            false,
        ),
        false,
    ),
    ty = BTreeMap<i64, bool>,
    values = [
        btreemap!{0 => true, 1 => false, 2 => true},
        btreemap!{3 => false, 4 => true},
        btreemap!{},
    ],
);

test_example!(
    test_name = flattened_structures,
    field = Field::new(
        "value",
        DataType::Struct(vec![
            Field::new("a", GenericDataType::I64, false),
            Field::new("b", DataType::Float32, false),
            Field::new("c", DataType::Float64, false),
        ]),
        false,
    )
    .with_metadata(strategy_meta(Strategy::MapAsStruct)),
    ty = Outer,
    values = [
        Outer {
            a: 0,
            inner: Inner { b: 1.0, c: 2.0 }
        },
        Outer {
            a: 3,
            inner: Inner { b: 4.0, c: 5.0 }
        },
        Outer {
            a: 6,
            inner: Inner { b: 7.0, c: 8.0 }
        },
    ],
    define = {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Outer {
            a: i64,
            #[serde(flatten)]
            inner: Inner,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        struct Inner {
            b: f32,
            c: f64,
        }
    },
);
 */

// TODO: fix these tests
/*
#[test]
fn nested_list_structs() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Item {
        a: Vec<Inner>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Inner {
        a: i8,
        b: i32,
    }

    let items = vec![
        Item {
            a: vec![Inner { a: 0, b: 1 }, Inner { a: 2, b: 3 }],
        },
        Item { a: vec![] },
        Item {
            a: vec![Inner { a: 4, b: 5 }],
        },
    ];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();

    let expected_fields = vec![field::large_list(
        "a",
        false,
        field::r#struct(
            "element",
            false,
            [field::int8("a", false), field::int32("b", false)],
        ),
    )];
    assert_eq!(fields, expected_fields);

    let values = serialize_into_arrays(&fields, &items).unwrap();

    let items_from_array: Vec<Item> = deserialize_from_arrays(&fields, &values).unwrap();
    assert_eq!(items_from_array, items);
}

#[test]
fn nested_structs_lists_lists() {
    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct Item {
        a: A,
        b: u16,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct A {
        c: Vec<C>,
    }

    #[derive(Debug, Deserialize, Serialize, PartialEq)]
    struct C {
        d: Vec<u8>,
    }

    let items = vec![
        Item {
            a: A {
                c: vec![C { d: vec![0, 1] }, C { d: vec![2] }],
            },
            b: 3,
        },
        Item {
            a: A { c: vec![] },
            b: 4,
        },
        Item {
            a: A {
                c: vec![C { d: vec![] }],
            },
            b: 5,
        },
    ];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let expected_fields = vec![
        field::r#struct(
            "a",
            false,
            [field::large_list(
                "c",
                false,
                field::r#struct(
                    "element",
                    false,
                    [field::large_list(
                        "d",
                        false,
                        field::uint8("element", false),
                    )],
                ),
            )],
        ),
        field::uint16("b", false),
    ];
    assert_eq!(fields, expected_fields);

    let arrays = serialize_into_arrays(&fields, &items).unwrap();
    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

#[test]
fn byte_arrays() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: Vec<u8>,
    }

    let items = vec![
        Item {
            a: b"hello".to_vec(),
        },
        Item {
            a: b"world!".to_vec(),
        },
    ];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();

    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

#[test]
fn new_type_structs() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: U64,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct U64(u64);

    let items = vec![Item { a: U64(21) }, Item { a: U64(42) }];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();

    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

macro_rules! define_wrapper_test {
    ($test_name:ident, $struct_name:ident, $init:expr) => {
        #[test]
        fn $test_name() {
            #[derive(Debug, PartialEq, Serialize, Deserialize)]
            struct $struct_name {
                a: u32,
            }

            let items = $init;

            let fields = serialize_into_fields(&items, Default::default()).unwrap();
            let arrays = serialize_into_arrays(&fields, &items).unwrap();

            let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

            assert_eq!(items_from_arrays, items);
        }
    };
}

define_wrapper_test!(
    wrapper_outer_vec,
    Item,
    vec![Item { a: 21 }, Item { a: 42 }]
);
define_wrapper_test!(
    wrapper_outer_slice,
    Item,
    [Item { a: 21 }, Item { a: 42 }].as_slice()
);
define_wrapper_test!(wrapper_const_array, Item, [Item { a: 21 }, Item { a: 42 }]);

#[test]
fn wrapper_tuple() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: u32,
    }

    let items = (Item { a: 21 }, Item { a: 42 });

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();

    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    let items = vec![items.0, items.1];
    assert_eq!(items_from_arrays, items);
}

#[test]
fn test_unit() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: (),
    }

    let items = vec![Item { a: () }, Item { a: () }];

    let fields =
        serialize_into_fields(&items, TracingOptions::default().allow_null_fields(true)).unwrap();
    let expected_fields = vec![Field::new("a", DataType::Null, true)];

    assert_eq!(fields, expected_fields);

    let arrays = serialize_into_arrays(&fields, &items).unwrap();
    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

#[test]
fn test_tuple() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: (u8, u8),
    }

    let items = vec![Item { a: (0, 1) }, Item { a: (2, 3) }];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();
    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

#[test]
fn test_tuple_struct() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: Inner,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Inner(u8, u8);

    let items = vec![Item { a: Inner(0, 1) }, Item { a: Inner(2, 3) }];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();
    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

#[test]
fn test_struct_with_options() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Item {
        a: Inner,
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Inner {
        foo: Option<u8>,
        bar: u32,
    }

    let items = vec![
        Item {
            a: Inner { foo: None, bar: 13 },
        },
        Item {
            a: Inner {
                foo: Some(0),
                bar: 21,
            },
        },
        Item {
            a: Inner {
                foo: Some(1),
                bar: 42,
            },
        },
    ];

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();
    let items_from_arrays: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items_from_arrays, items);
}

#[test]
fn test_complex_benchmark_example() {
    use rand::{
        distributions::{Distribution, Standard, Uniform},
        Rng,
    };

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct Item {
        string: String,
        points: Vec<(f32, f32)>,
        float: Float,
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    enum Float {
        F32(f32),
        F64(f64),
    }

    impl Item {
        fn random<R: Rng + ?Sized>(rng: &mut R) -> Self {
            let n_string = Uniform::new(1, 20).sample(rng);
            let n_points = Uniform::new(1, 20).sample(rng);
            let is_f32: bool = Standard.sample(rng);

            Self {
                string: (0..n_string)
                    .map(|_| -> char { Standard.sample(rng) })
                    .collect(),
                points: (0..n_points)
                    .map(|_| (Standard.sample(rng), Standard.sample(rng)))
                    .collect(),
                float: if is_f32 {
                    Float::F32(Standard.sample(rng))
                } else {
                    Float::F64(Standard.sample(rng))
                },
            }
        }
    }

    let mut rng = rand::thread_rng();
    let items: Vec<Item> = (0..10).map(|_| Item::random(&mut rng)).collect();

    let fields = serialize_into_fields(&items, Default::default()).unwrap();
    let arrays = serialize_into_arrays(&fields, &items).unwrap();

    let round_tripped: Vec<Item> = deserialize_from_arrays(&fields, &arrays).unwrap();

    assert_eq!(items, round_tripped);
}
*/
