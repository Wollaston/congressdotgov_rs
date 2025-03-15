macro_rules! assert_missing_field {
    ($err:expr_2021, $type:tt, $field:expr_2021 $(,)?) => {
        let in_err = $err;
        let in_field = $field;
        if let $type::UninitializedField(field) = in_err {
            assert_eq!(field, in_field);
        } else {
            panic!(
                "unexpected error (expected to be missing `{}`): {:?}",
                in_field, in_err,
            );
        }
    };
}

pub(crate) use assert_missing_field;
