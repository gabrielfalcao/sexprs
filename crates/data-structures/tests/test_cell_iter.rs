use k9::assert_equal;
use sexprs_data_structures::{Cell, Value};

#[test]
fn test_cell_into_iterator() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));

    let strings = cell
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>();
    assert_equal!(strings, vec!["a", "b", "c"]);
}

#[test]
fn test_cell_from_iterator_quoted_list() {
    let list = Value::list({
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    });

    let cell = Cell::from_iter(list);

    assert_equal!(cell, {
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    });
}

#[test]
fn test_cell_is_empty() {
    let cell = Cell::nil();

    assert_equal!(cell.is_empty(), true);
    let cell = {
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    };
    assert_equal!(cell.is_empty(), false);
    let cell = {
        let mut cell = Cell::nil();
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    };
    assert_equal!(cell.is_empty(), false);
}

// #[test]
// fn test_cell_from_iterator_cell() {
//     let mut cell = Cell::from("a");
//     cell.add(&Cell::from("b"));
//     cell.add(&Cell::from("c"));

//     let value = Value::from_iter(cell);

//     assert_equal!(
//         value,
//         Value::from({
//             let mut cell = Cell::from("a");
//             cell.add(&Cell::from("b"));
//             cell.add(&Cell::from("c"));
//             cell
//         })
//     );
// }

// #[test]
// fn test_cell_extend_value_list() {
//     let mut value = Value::from({
//         let mut cell = Cell::from("a");
//         cell.add(&Cell::from("b"));
//         cell.add(&Cell::from("c"));
//         cell
//     });
//     let list_xyz = Value::from({
//         let mut cell = Cell::from("x");
//         cell.add(&Cell::from("y"));
//         cell.add(&Cell::from("z"));
//         cell
//     });

//     value.extend(list_xyz.into_iter());

//     assert_equal!(
//         value,
//         Value::from({
//             let mut cell = Cell::from("a");
//             cell.add(&Cell::from("b"));
//             cell.add(&Cell::from("c"));
//             cell.add(&Cell::from("x"));
//             cell.add(&Cell::from("y"));
//             cell.add(&Cell::from("z"));
//             cell
//         })
//     );
// }

// #[test]
// fn test_cell_extend_cell() {
//     let mut value = Value::from({
//         let mut cell = Cell::from("a");
//         cell.add(&Cell::from("b"));
//         cell.add(&Cell::from("c"));
//         cell
//     });
//     let mut cell = Cell::from("x");
//     cell.add(&Cell::from("y"));
//     cell.add(&Cell::from("z"));

//     value.extend(cell.into_iter());

//     assert_equal!(
//         value,
//         Value::from({
//             let mut cell = Cell::from("a");
//             cell.add(&Cell::from("b"));
//             cell.add(&Cell::from("c"));
//             cell.add(&Cell::from("x"));
//             cell.add(&Cell::from("y"));
//             cell.add(&Cell::from("z"));
//             cell
//         })
//     );
// }
