const API_URL = "http://127.0.0.1:3000"



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
       };
    }
}

export async function getComments(post_list){
     const info = {
        "posts" : post_list,
     };
    const location = API_URL + `/getComments`;
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

export async function changeColor(token, color){
    const info = {
        "token" : token,
        "color" : color
    };
    const location = API_URL + `/changeColor`;
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

export async function star(post, token){}

export async function edit_comment(comment_id,token,content){
    const info = {
        "comment_id" : comment_id,
        "token" : token,
        "content" : content,
    };

    const location = API_URL + `/editComment`;
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

export async function delete_comment(comment_id,token){
     const info = {
        "comment_id" : comment_id,
        "token" : token,
    };

    const location = API_URL + `/deleteComment`;
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

export async function post_comment(token,content){
     const info = {
        "token" : token,
        "content" : content,
    };

    const location = API_URL + `/postComment`;
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

export async function subscribe_shouts(){
    const event_source = new EventSource("/subscribeShouts");
    
   
    event_source.onmessage = (ev) => {
      // default "message" event
      addMessage(JSON.parse(ev.data));
    };
    
    event_source.onerror = () => {
      console.log("shouts disconnected, retrying…");
    }; 
}
export async function get_shouts(shouts_before_date){
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

export async function post_shout(token,content){
    const info = {
        "token" : token,
        "content" : content,
    };

    const location = API_URL + `/postShout`;
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

export async function edit_shout(shout_id,token,content){
    const info = {
        "shout_id" : shout_id,
        "token" : token,
        "content" : content,
    };

    const location = API_URL + `/editShout`;
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

export async function delete_comment(comment_id,token){
     const info = {
        "shout_id" : shout_id,
        "token" : token,
    };

    const location = API_URL + `/deleteShout`;
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

