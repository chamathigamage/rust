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
    use std::time::{SystemTime};
    use mockall::*;
    pub struct Cham<'a> {
        pub pet: &'a str,
    }

    #[automock]
    trait MyPet {
        fn getPet(&self) -> Result<String, &'static str>;
    } 
    
    let me = Cham {
        pet: "Amaya",
    };

    let mut mock = MockMyPet::new();
    mock.expect_getPet()
        .returning(move || if me.pet != "Amaya" {
                println!("{:?}", SystemTime::now());
                Ok(format!("This is my pet dog Peggy"))
            } else {
                Err("NOOO")
            }
        );

    impl MyPet for Cham<'_> {
        fn getPet(&self) -> Result<String, &'static str> {
            Ok(format!("This is my pet cat {}", self.pet))
        }
    }
    println!("{:?}", mock.getPet());
    println!("{:?}", me.getPet());

    
    async fn check_correct_pet() -> Result<(), String> {
        let start = SystemTime::now();
        again::retry(|| async {
            mock.getPet()
        }).await.unwrap();
        let end = SystemTime::now();
        println!("{:?}", end.duration_since(start) );
        Ok(())
    }
}




