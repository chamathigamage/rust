// fn main(){
//     struct Bar{}
//     use mockall::*;
//     // use mockall::predicate::*;
//     #[automock(type Key=u16; type Value=i32;)]
//     pub trait A {
//         type Key;
//         type Value;
//         fn foo(&self, k: Self::Key) -> Self::Value;
//     }

//     let mut mock = MockA::new();
//     mock.expect_foo()
//         .returning(|x: u16| i32::from(x));

//     if mock.foo(4)==4 {
//         println!("YESS");
//     }

//     impl Bar {
//         fn foo(&mut self) {
//             println!("In struct impl!")
//         }
//     }

//     let mut b = Bar{};
//     b.foo();
// }

// fn main() {
//     use mockall::*;
//     #[automock]
//     trait MyTrait {
//         fn foo(&self, x: str) -> str;
//     }

//     // impl Bar {
//     //     fn foo(&mut self) {
//     //         println!("In struct impl!")
//     //     }
//     // }

//     // let mut b = Bar{};
//     // b.foo();
//     // fn call_with_four(x: &dyn MyTrait) -> u32 {
//     //     x.foo("")
//     // }

//     let mut mock = MockMyTrait::new();
//     mock.expect_foo()
//         .returning("this is a mock");


// }



fn main(){

    use mockall::*;
    pub struct Cham<'a> {
        pub pet: &'a str,
    }

    #[automock]
    trait MyPet {
        fn getPet(&self) -> String;
    }

    let mut mock = MockMyPet::new();
    mock.expect_getPet()
        .returning(|| (format!("This is my pet dog Peggy")));

    impl MyPet for Cham<'_> {
        fn getPet(&self) -> String {
            format!("This is my pet cat {}", self.pet)
        }
    }

    let me = Cham {
        pet: "Amaya",
    };

    println!("{}", me.getPet());
    println!("{}", mock.getPet());

}