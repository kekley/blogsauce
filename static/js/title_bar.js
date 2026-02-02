const SPLASH_ENDPOINT= "http://127.0.0.1:3000/getSplash"


export async function get_random_window_title(){
    const response = await fetch(SPLASH_ENDPOINT);
    const body = await response.text();
    const response_json = JSON.parse(body);
    if ("splash" in response_json){
        return response_json["splash"];
    }
    return "";
}
