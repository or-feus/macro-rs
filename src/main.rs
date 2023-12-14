
#[allow(unused_macros)]
macro_rules! say_hello {
    () => {
        println!("Hello!");
    };
}

#[allow(unused_macros)]
macro_rules! create_function {
    ($func_name:ident) => {
        fn $func_name(){
            println!("Function {:?} is called", stringify!($func_name))
        }
    };
}


#[allow(unused_macros)]
macro_rules!  calculate {
    (add $a:expr, $b:expr) => {
        println!("{} + {} = {}", $a, $b, $a + $b);
    };

    (sub $a:expr, $b:expr) => {
        println!("{} - {} = {}", $a, $b, $a - $b);
    }

}

#[allow(unused_macros)]
macro_rules! array {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(temp_vec.push($x);)*
            temp_vec
        }
    };
}

#[allow(unused_macros)]
macro_rules! check {
    ($x:expr) => {
        if $x {
            println!("Condition met!");
        } else {
            println!("Condition not met!");
        }
    }
}

#[allow(unused_macros)]
macro_rules! html {
    ( $tag:ident { $($body:tt)* } ) => {
        format!("<{0}>{1}</{0}>", stringify!($tag), html!($($body)*))
    };
    ( $text:expr) => {
        $text.to_string()
    };
}


#[allow(unused_macros)]
macro_rules! vec_map {
    ($vec:expr, $func: expr) => {
        $vec.into_iter().map($func).collect::<Vec<_>>()
    };
}

#[allow(unused_macros)]
macro_rules! declare_variables {
    ($($name:ident : $type:ty), *) => {
        $(let $name: $type;)*
    };
}


#[allow(unused_macros)]
macro_rules! match_types {
    ($val:expr, int) => {
        println!("{} is int", $val)
    };
    ($val:expr, float) => {
        println!("{} is float", $val)
    };
    ($val:expr, str) => {
        println!("{} is str", $val)
    };
    
}


fn main() {
    
    match_types!(32, int);
    match_types!(3.14, int);
    match_types!("hello", int);
}