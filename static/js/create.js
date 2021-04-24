let createButton = document.getElementById("create-button");

console.assert(createButton);

createButton.addEventListener("click", async () => {

    let qrCodeContainer = document.getElementById("display-qrcode");
    let secretKeyContainer = document.getElementById("secret-key");

    console.assert(qrCodeContainer);
    console.assert(secretKeyContainer);

    qrCodeContainer.innerHTML = null;
    secretKeyContainer.innerHTML = null;
    createButton.disabled = true;

    try {
        let response = await fetch("api/create");

        console.log(response);

        if (!response.ok) {
            // TODO
        }

        let jsonResponse = await response.json();

        console.log(jsonResponse);
        console.assert(jsonResponse.qrcode);
        console.assert(jsonResponse.secret_key);

        qrCodeContainer.innerHTML = jsonResponse.qrcode;
        secretKeyContainer.innerHTML = jsonResponse.secret_key;
    } catch (e) {
        // TODO
        console.error(e);
    } finally {
        createButton.disabled = false;
    }
});

