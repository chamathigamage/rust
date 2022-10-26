#[tokio::main]
async fn main(){
    use std::time::{SystemTime, Duration, SystemTimeError};
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

    
    async fn check_correct_pet(mock: &MockMyPet) -> Result<Duration, SystemTimeError> {
        let retry_policy = again::RetryPolicy::exponential(core::time::Duration::from_secs(1))
        .with_max_delay(core::time::Duration::from_secs(120))
        .with_max_retries(1);
        println!{"Starting retries"}
        let start = SystemTime::now();
        let result = retry_policy.retry(|| async {
            mock.getPet()
        }).await;
        let end = SystemTime::now();
        end.duration_since(start)
    }

    let return_result = check_correct_pet(&mock).await.unwrap();
    println!("{:?}", return_result);
}




