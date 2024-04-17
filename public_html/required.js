// some required stuff like screen adjustments, html loading etc.

var menuState = true;
var Timer = 0;


// Adjust screen size for mobile and 4k monitors for some reason
function adjustScreenSize() {
    let screenWidth = window.innerWidth || document.documentElement.clientWidth || document.body.clientWidth;
    let referenceWidth = 1920;
    let referenceFontSize = 16;
    let phoneReferenceWidth = 700;

    if (screenWidth >= referenceWidth) { // 2000 ++
        let fontSize = (screenWidth / referenceWidth) * referenceFontSize;
        document.documentElement.style.fontSize = fontSize + 'px';

    } else if (screenWidth >= phoneReferenceWidth) { // 720 - 2000
        document.documentElement.style.fontSize = referenceFontSize + 'px';

    } else { // -- 720
        let fontSize = (screenWidth / phoneReferenceWidth) * referenceFontSize;
        document.documentElement.style.fontSize = fontSize + 'px';
    }

    if (screenWidth < "370") {
        document.getElementById("headerIcon").src = "/_images/Obliteration-smoll.png";
    } else {
        document.getElementById("headerIcon").src = "/_images/Obliteration.png";
    }
}

function headerUpdate() {
    requestAnimationFrame(headerShadow)
    // headerShadow()
}


function headerShadow() {
    const header = document.querySelector('header');

    if (window.scrollY > 0) {
        header.style.boxShadow = "0 1px 5px rgb(0, 0, 0, 0.2)";
    } else {
        header.style.boxShadow = '';
    }
}


function menuButton() {
    const menu = document.getElementById("menu");
    const menuButton = document.getElementById("menuButton");
    clearTimeout(Timer);

    if (menuState === true) {
        menuState = false;
        menuButton.src = "/_images/close.svg";
        menu.style.display = "flex";
        menu.style.opacity = 0;

        Timer = setTimeout(() => {
            menu.style.opacity = 1;
        }, 10);
    } else if (menuState === false) {
        menuState = true;
        menuButton.src = "/_images/menu.svg";
        menu.style.opacity = 0;

        Timer = setTimeout(() => {
            menu.style.display = "none";
        }, 200);
    }
}

function offMenu() {
    menuButton();
}


function init() {
    let header_html = `
        <header class="header">
            <a class="headerIcon" href="/">
                <img id="headerIcon" src="/_images/Obliteration.png" height="40px" width="auto" alt="Obliteration logo">
            </a>
            <div class="headerRight">
                <a class="headerLink" href="./download">Download</a>
                <a class="headerLink" href="./compatibility">Compatibility</a>
                <a class="headerLink" href="./wiki">Wiki</a>
                <div class="headerRightIcons">
                    <a href="https://github.com/obhq/obliteration" target="_blank" style="display: flex;">
                        <img class="headerRightIconGithub" src="/_images/github.svg" alt="GitHub logo">
                    </a>
                    <a href="https://discord.gg/Qsdaxj6tnH" target="_blank" style="display: flex;">
                        <img class="headerRightIconDiscord" src="/_images/discord.svg" alt="Discord logo">
                    </a>
                    <img class="headerRightIconMenu" src="/_images/menu.svg" onclick="menuButton()" id="menuButton" alt="Mobile menu button">
                </div>
            </div>
        </header>`;

    let menu_html = `
        <div class="menuContainer" id="menu" onclick="offMenu()">
            <div class="menu">
                <a class="menuLink" href="./download">Download</a>
                <a class="menuLink" href="./compatibility">Compatibility</a>
                <a class="menuLink" href="https://github.com/obhq/obliteration/wiki">Wiki</a>
            </div>
        </div>`;

    document.getElementById("header").outerHTML = header_html;
    document.getElementById("menu").outerHTML = menu_html;
}