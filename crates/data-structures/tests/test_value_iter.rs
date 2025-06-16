use k9::assert_equal;
use sexprs_data_structures::{Cell, Value};

#[test]
fn test_value_list_into_iterator() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));

    let value = Value::list(cell);

    assert_equal!(
        value,
        Value::from({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell
        })
    );

    let strings = value
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>();
    assert_equal!(strings, vec!["a", "b", "c"]);
}

#[test]
fn test_value_quoted_list_into_iterator() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));

    let value = Value::quoted_list(cell);

    assert_equal!(
        value,
        Value::quoted_list({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell
        })
    );

    let strings = value
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>();
    assert_equal!(strings, vec!["a", "b", "c"]);
}

#[test]
fn test_value_list_from_iterator_quoted_list() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));
    let quoted_list = Value::quoted_list(cell);

    let value = Value::from_iter(quoted_list);

    assert_equal!(
        value,
        Value::list({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell
        })
    );
}

#[test]
fn test_value_list_from_iterator_cell() {
    let mut cell = Cell::from("a");
    cell.add(&Cell::from("b"));
    cell.add(&Cell::from("c"));

    let value = Value::from_iter(cell);

    assert_equal!(
        value,
        Value::from({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell
        })
    );
}

#[test]
fn test_value_list_extend_value_list() {
    let mut value = Value::from({
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    });
    let list_xyz = Value::from({
        let mut cell = Cell::from("x");
        cell.add(&Cell::from("y"));
        cell.add(&Cell::from("z"));
        cell
    });

    value.extend(list_xyz.into_iter());

    assert_equal!(
        value,
        Value::from({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell.add(&Cell::from("x"));
            cell.add(&Cell::from("y"));
            cell.add(&Cell::from("z"));
            cell
        })
    );
}


#[test]
fn test_value_list_extend_cell() {
    let mut value = Value::from({
        let mut cell = Cell::from("a");
        cell.add(&Cell::from("b"));
        cell.add(&Cell::from("c"));
        cell
    });
    let mut cell = Cell::from("x");
    cell.add(&Cell::from("y"));
    cell.add(&Cell::from("z"));

    value.extend(cell.into_iter());

    assert_equal!(
        value,
        Value::from({
            let mut cell = Cell::from("a");
            cell.add(&Cell::from("b"));
            cell.add(&Cell::from("c"));
            cell.add(&Cell::from("x"));
            cell.add(&Cell::from("y"));
            cell.add(&Cell::from("z"));
            cell
        })
    );
}
