// This is the same for all of the below, and
// you probably won't need it except for debugging
// in most cases.
function bytesToHex(bytes) {
    return Array.from(
      bytes,
      byte => byte.toString(16).padStart(2, "0")
    ).join("");
  }
  
  // You almost certainly want UTF-8, which is
  // now natively supported:
  function stringToUTF8Bytes(string) {
    return new TextEncoder().encode(string);
  }
  

var redirect = function (url) {
     
     setTimeout(function() { window.location.href = url; }, 1000);
}


var resolve_urs = function (urs_link){
    api_url = '/resolve/' + urs_link ;


    var url = window.location.ancestorOrigins[0];


    $.getJSON(api_url, function(data) {
            // Suponiendo que el API devuelve un objeto con una propiedad 'message'
            //let message = data.message;

            if (!jQuery.isEmptyObject(data)) {
                let emanifest = bytesToHex(stringToUTF8Bytes(data.endpoint));
                let redirect_url = `${url}/html/hex/${emanifest}`;

                
                window.open(redirect_url, '_blank');

                //Swal.fire('Cita verificada', redirect_url, 'success');

                

            }else {
                Swal.fire('Cita', "No se ha podido resolver el enlace");
            }

        })
        .fail(function() {
            Swal.fire('Cita', "No se ha podido resolver el enlace");
        });
}

var resolve_urs_locator = function (urs_link){
    api_url = '/locate/' + urs_link ;

    console.log(api_url);

    $.get(api_url, function(data) {
            // Suponiendo que el API devuelve un objeto con una propiedad 'message'
            //let message = data.message;
            console.log(data);

            if (data.length > 0) {
                Swal.fire('Contenido referenciado', data, 'success');
            }else {
                Swal.fire('Cita', "No se ha podido resolver el enlace");
            }

        })
        .fail(function() {
            Swal.fire('Cita', "No se ha podido resolver el enlace");
        });
}