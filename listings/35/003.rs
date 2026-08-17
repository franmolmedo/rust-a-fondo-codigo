let request_id = String::from("req-42");
let future = async move {
    log_request(&request_id).await;
};
