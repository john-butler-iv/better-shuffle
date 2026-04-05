#[allow(unused_variables, dead_code)]
#[get("/authorize?response_type=code&<client_id>&<scope>&<redirect_uri>&<state>")]
fn authorize(client_id: &str, scope: &str, redirect_uri: &str, state: &str) {}
