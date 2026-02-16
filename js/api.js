const API_URL = "https://stuff.kekley.online"



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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
    } catch (e) {
       console.log(e);
       return {
           "error": `${e}`
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
        const response_json = await response.json();
        console.log(response_json);
        return response_json;
           
        } catch (e) {
           console.log(e);
          return {
           "error": `${e}`
       };

    }

}

export async function register_display_name(display_name){
     if(display_name.length ==0){
         return {
             "error" : "username cannot be empty"
         };
     }
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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
           "error": `${e}`
       }; 
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
        const response_json = await response.json();
        console.log(response_json);
        return response_json;
        } catch (e) {
           console.log(e);
           return {
           "error": `${e}`
           };
    }

}

export async function star(post, token){
    const info = {
        "token" : token,
        "post" : post,
    };

    const location = API_URL + `/star`;
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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
           "error": `${e}`
       };
    }

}

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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
           "error": `${e}`
       };
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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
        "error" : `${e}`
       };
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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
        "error" : `${e}`
       };
    }
}

export function subscribe_shouts(callback,token) {
    let token_param = "";
    if(token!=undefined && token!=null){
        token_param=`?token=${token}`
    }
  const es = new EventSource(API_URL + `/subscribeShouts${token_param}`);

  es.onopen = () => {
    console.log("SSE connected");
  };

  es.onmessage = (ev) => {
    callback(JSON.parse(ev.data));
  };

  es.onerror = () => {
    console.log(
      "SSE state:",
      es.readyState === EventSource.CONNECTING
        ? "reconnecting"
        : "closed"
    );
  };
}



export async function get_shouts(shouts_before_id,token){
    let info = {
    };
    if(shouts_before_id != undefined && shouts_before_id !=null){
        info["shouts_before_id"] = shouts_before_id;
    }
    console.log(token);
    if(token != undefined && token!=null){
        console.log(token);
        info["token"] =token;
    }
 
    const location = API_URL + `/getShouts`;
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
    const response_json = await response.json();
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
    const response_json = await response.json();
    console.log(JSON.stringify(response_json));
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
        "error" : `${e}`
       };
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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
        "error" : `${e}`
       };
    }

}

export async function delete_shout(comment_id,token){
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
    const response_json = await response.json();
    console.log(response_json);
    return response_json;
       
    } catch (e) {
       console.log(e);
       return {
        "error" : `${e}`
       };
    }
}

