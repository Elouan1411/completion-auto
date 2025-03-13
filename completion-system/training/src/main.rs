// #[tokio::main]
// async fn main() {
//     let token = tokio_util::sync::CancellationToken::new();
//     let cloned_token = token.clone();

//     let task1_handle = tokio::spawn(async move {
//         tokio::select! {
//             _ = cloned_token.cancelled() => {}
//             _ = async {
//                 loop {
//                     println!("task 1 hit");
//                     tokio::time::sleep(std::time::Duration::from_secs(2)).await;
//                 }
//             } => {}
//         }
//     });

//     tokio::spawn(async move {
//         // Ici je simule un click au bout de 10 secondes, au lieu de ca tu met ton code qui fait
//         // que ca coupe le(s) thread(s)
//         tokio::time::sleep(std::time::Duration::from_secs(10)).await;

//         token.cancel();
//     });

//     task1_handle.await.unwrap()
// }


#[tokio::main]
async fn main() {
    let token = tokio_util::sync::CancellationToken::new();
    let cloned_token = token.clone();

    let task1_handle = tokio::spawn(async move {
        tokio::select! {
            _ = cloned_token.cancelled() => {}
            _ = async {
                loop {
                    println!("task 1 hit");
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                }
            } => {}
        }
    });

    tokio::spawn(async move {
        // Ici je simule un click au bout de 10 secondes, au lieu de ca tu met ton code qui fait
        // que ca coupe le(s) thread(s)
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;

        token.cancel();
    });

    task1_handle.await.unwrap()
}
