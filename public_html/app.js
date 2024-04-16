// Adjust screen size for mobile and 4k monitors for some reason
function adjustScreenSize() {
    let screenWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
    let referenceWidth = 1920;
    let referenceFontSize = 16;
    let phoneReferenceWidth = 720;

    // document.documentElement.style.width = "";
    if (screenWidth >= referenceWidth) { // 2000 ++
        let fontSize = (screenWidth / referenceWidth) * referenceFontSize;
        document.documentElement.style.fontSize = fontSize + 'px';

    } else if (screenWidth >= phoneReferenceWidth) { // 720 - 2000
        document.documentElement.style.fontSize = referenceFontSize + 'px';

    } else { // -- 720
        let fontSize = (screenWidth / phoneReferenceWidth) * referenceFontSize;
        //   document.documentElement.style.width = screenWidth + "px";
        let viewport = document.querySelector('meta[name="viewport"]');
        viewport.content = "initial-scale=1";
        document.documentElement.style.fontSize = fontSize + 'px';
    }

    if (screenWidth < "370") {
        document.getElementById("headerIcon").src = "/_images/Obliteration-smoll.png";
    } else {
        document.getElementById("headerIcon").src = "/_images/Obliteration.png";
    }
}

// Adjust screen size for mobile and 4k monitors for some reason
window.addEventListener('load', adjustScreenSize);
window.addEventListener('resize', adjustScreenSize);