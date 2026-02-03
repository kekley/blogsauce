const API_URL = "http://127.0.0.1:3000"

export async function get_shouts(){
    //TODO add the date field
    const info = {};
    const location = API_URL + `/getShouts`;
    const response = await fetch(location, {
        method: "GET",
        mode: "cors",
        headers: {
            "Content-Type" : "application/json",
        }
        ,
        body: JSON.stringify(info),
    });
    try {
     if(!response.ok){
        throw new Error(`Response status: ${response.status}`);
    }
    const response_json = response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
    }
}

export async function get_splash(){
    const location = API_URL + `/getSplash`;
    const response = await fetch(location,{
        method: "GET",
        mode: "cors",
    });
    const body = await response.text();
    const response_json = JSON.parse(body);
    if ("splash" in response_json){
        return response_json["splash"];
    }
    return "";
}

export async function verify_token(token){
      const info = {
        "token" : token,
     };
    
    const location = API_URL + `/verifyToken`;
    const response = await fetch(location, {
        method: "POST",
        mode: "cors",
        headers: {
            "Content-Type" : "application/json",
        }
        ,
        body: JSON.stringify(info),
    });
    try {
     if(!response.ok){
         return {
            "error" : `Response status: ${response.status}`
         };
    }
    const response_json = response.json();
    console.log(response_json);
    return response_json;
    } catch (e) {
       console.log(e);
       return {
            "error" : "unknown"
       };
    }
}

export async function getComments(post_list){
    
}


export async function register_display_name(display_name){
     const info = {
        "display_name" : display_name,
     };
    const location = API_URL + `/registerName`;
    const response = await fetch(location, {
        method: "POST",
        mode: "cors",
        headers: {
            "Content-Type" : "application/json",
        }
        ,
        body: JSON.stringify(info),
    });
    try {
     if(!response.ok){
        throw new Error(`Response status: ${response.status}`);
    }
    const response_json = response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {};
    }   
}
