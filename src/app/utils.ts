export async function sendGetRequest(url: string, func: any) {
    var http = new XMLHttpRequest();
    http.onreadystatechange = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            func(this.responseText);
        }
    }

    http.open("GET", url, true);
    http.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
    http.send();
}

export async function sendPostRequest(url: string, body: any, func: any) {
    var http = new XMLHttpRequest();
    http.onload = function() {
        if ((this.readyState == 4) && (this.status == 200)) {
            func(this.responseText);
        }
    }

    http.open("POST", url, true);
    http.setRequestHeader("Content-Type", "application/json");
    http.send(JSON.stringify(body));
}

export function parseResponse(r: any) {
    let result = JSON.parse(r);
    if (result['type'] != "success") {
        console.log('something wrong happened');
        console.log(r);
        // window.location.href = `/soterius/login.html?redirect=${encodeURIComponent(window.location)}`;
        // or some kind of proper error handling
    }

    return decodeURIComponent(result['data']);
}


export function fetchCookie(name: string) {
    var result = undefined;
    document.cookie.split(';').forEach(element => {
        let x = element.trim().split("=");
        if (x[0] == name) {
            result = x[x.length - 1];
        }
    });
    return result;
}
