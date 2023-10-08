var resolve_urs = function (urs_link){
    api_url = 'https://3001-jredrado-rusttemplate-8sepdmxsit1.ws-eu105.gitpod.io/pub/' + urs_link + '/resolve';

    $.getJSON(api_url, function(data) {
            // Suponiendo que el API devuelve un objeto con una propiedad 'message'
            //let message = data.message;
            console.log(data);
            if (data.length > 0) {
                Swal.fire('Cita verificada', data[0].endpoint, 'success');
            }else {
                Swal.fire('Cita', "No se ha podido resolver el enlace");
            }

        })
        .fail(function() {
            Swal.fire('Cita', "No se ha podido resolver el enlace");
        });
}