let getURLQueryParams = function () {
    let params = {};
    let query = new URL(document.currentScript.src).search;
    if (query && query.length) {
        query = query.substring(1);
        let keyParams = query.split('&');
        for (let x = 0; x < keyParams.length; x++) {
            let keyVal = keyParams[x].split('=');
            if (keyVal.length > 1) {
                params[keyVal[0]] = decodeURIComponent(keyVal[1]);
            }
        }
    }
    return params;
};
let urlParams = getURLQueryParams();

let manifest_url = new URL(urlParams['url']);
let id = urlParams['id'];


if ('registerProtocolHandler' in navigator) {
    navigator.registerProtocolHandler('web+urs', `${document.location.origin}/viewer/index_dita.html?id=${id}&urs=%s&proxy=${encodeURIComponent(manifest_url.origin)}`);
}