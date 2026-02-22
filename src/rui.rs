// src/rui.rs
#[macro_export]
macro_rules! rui {
    // 1. Main rule: process a widget name and its fields
    ($name:ident { $($fields:tt)* }) => {{
        let mut builder = $crate::widget::$name::builder();
        $crate::rui!(@parse builder, $($fields)*);
        builder.build()
    }};

    // 2. Internal rule: process field 'children' (recursively)
    (@parse $builder:ident, children: [ $($child_name:ident { $($child_fields:tt)* }),* $(,)? ] $(, $($rest:tt)*)?) => {
        $(
            $builder = $builder.add_child(Box::new($crate::rui! { $child_name { $($child_fields)* } }));
        )*
        $crate::rui!(@parse $builder, $($($rest)*)?);
    };

    // 3. Internal rule: process regular fields (x, y, color etc.)
    (@parse $builder:ident, $key:ident : $val:expr $(, $($rest:tt)*)?) => {
        $builder = $builder.$key($val);
        $crate::rui!(@parse $builder, $($($rest)*)?);
    };

    // 4. Empty rule when there are no more fields to process
    (@parse $builder:ident, ) => {};
}

