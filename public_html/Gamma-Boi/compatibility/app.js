let databaseJsonData; // :3
let totalPages;
let tagFilter = [];
let oldestFilter = false;

// Check Avif Support
console.log(`Hey there! I've implemented an avif support check to spare browsers like Edge from having a stroke :D!`);
const avif = new Image();
avif.src = "data:image/avif;base64,AAAAIGZ0eXBhdmlmAAAAAGF2aWZtaWYxbWlhZk1BMUIAAADybWV0YQAAAAAAAAAoaGRscgAAAAAAAAAAcGljdAAAAAAAAAAAAAAAAGxpYmF2aWYAAAAADnBpdG0AAAAAAAEAAAAeaWxvYwAAAABEAAABAAEAAAABAAABGgAAAB0AAAAoaWluZgAAAAAAAQAAABppbmZlAgAAAAABAABhdjAxQ29sb3IAAAAAamlwcnAAAABLaXBjbwAAABRpc3BlAAAAAAAAAAIAAAACAAAAEHBpeGkAAAAAAwgICAAAAAxhdjFDgQ0MAAAAABNjb2xybmNseAACAAIAAYAAAAAXaXBtYQAAAAAAAAABAAEEAQKDBAAAACVtZGF0EgAKCBgANogQEAwgMg8f8D///8WfhwB8+ErK42A=";
avif.onload = function () {
    console.log('AVIF IS SUPPORTED :D');
    avifSupport = true;
};
avif.onerror = function () {
    console.log('AVIF IS NOT SUPPORTED D:');
    avifSupport = false;
    window.alert("Hey! Your browser doesn't support avif, avif is an image format that has low file sizes while having high quality images. You won't get any game images");
};


document.addEventListener('DOMContentLoaded', function () {
    init();
    adjustScreenSize();
    headerShadow();

    window.addEventListener('resize', adjustScreenSize);
    window.addEventListener("scroll", headerUpdate);

    // + fetch issues and set the tag bars
    fetch('../../../updater/database.json')  //todo : change lmao
        .then(response => response.json())
        .then(jsonData => {
            gameCardHandler(jsonData);
            databaseJsonData = jsonData;
            // pageButtonHandler();

            totalIssues = jsonData.length;
            totalPages = totalIssues / 20;


            // stats & tag filter
            console.log("\nCOMPATIBILITY STATS");

            let availableTags = ['Nothing', 'Boots', 'Intro', 'Ingame', 'Playable'];
            let totalPercentage = 0;
            let naCount = 0;

            let tagPercentages = [];
            let tagCount = [];

            // get tag count
            availableTags.forEach(tag => {
                // init tag count
                tagCount[tag] = 0;

                jsonData.forEach(issue => {
                    if (issue.tag === tag) {
                        tagCount[tag]++;

                    } else if (issue.tag === "N/A") {
                        naCount++;
                    }
                });
            });

            // get raw tag percentages
            availableTags.forEach(tag => {
                let rawPercentage = (tagCount[tag] / (totalIssues - naCount)) * 100;

                tagPercentages[tag] = rawPercentage.toFixed(2);
                totalPercentage += rawPercentage.toFixed(2);
            });

            // make percentages correct (mostly ~heh)
            if (totalPercentage !== 100) {
                let difference = (100 - totalPercentage) / availableTags.length;

                availableTags.forEach(tag => {
                    tagPercentages[tag] = parseFloat(tagPercentages[tag] + difference).toFixed(2);
                });
            }


            availableTags.forEach(tag => {
                var percent = tagPercentages[tag];
                var count = tagCount[tag];

                var tagBar = document.getElementById(tag + 'Bar');
                var tagContainer = tagBar.parentElement;
                console.log(`${tag} = ${percent}% [${count}]`);

                tagBar.style.width = percent + '%';
                document.getElementById(tag + 'Percent').textContent = percent + '%';
                document.getElementById(tag + 'Number').textContent = count;

                tagContainer.addEventListener('click', function () {
                    tagContainer.classList.toggle('compStatusSelected');
                    tagFilter.includes(tag) ? tagFilter.splice(tagFilter.indexOf(tag), 1) : tagFilter.push(tag);

                    pageNumber = 1;
                    updateSearchResults();
                });
            });

            console.log("\n");

        })
        .catch(console.error);
});

// Searching
document.getElementById('search').addEventListener('input', function () {
    pageNumber = 1;
    updateSearchResults();
});

function updateSearchResults() {
    clearTimeout(Timer);
    const cardsContainer = document.getElementById("CardsContainer");
    const searchQuery = document.getElementById('search').value.toLowerCase();

    // skeleton animation
    cardsContainer.querySelectorAll(".gameCardS, .gameCard, .gameCardE").forEach(container => {
        const skeletonDiv = document.createElement('a');

        container.classList.forEach(name => {
            skeletonDiv.classList.add(name)
        });

        skeletonDiv.classList.add("skeletonLoading");
        cardsContainer.replaceChild(skeletonDiv, container);
    });

    Timer = setTimeout(() => {
        var jsonData = [];

        databaseJsonData.forEach(game => {
            if (!game.title.toLowerCase().includes(searchQuery)) {
                return;
            }

            if (tagFilter.length > 0) {
                let isGood = false;

                tagFilter.forEach(tag => {
                    if (tag === game.tag) {
                        isGood = true;
                    }
                })

                if (isGood === false) {
                    return;
                }
            }

            jsonData.push(game);
        });

        gameCardHandler(jsonData);
    }, 300);
}


// Game Card handler
function gameCardHandler(jsonData) {
    const gameWrapper = document.getElementById("CardsContainer");
    gameWrapper.innerHTML = "";

    let currentIssue = 0;
    let totalIssues = jsonData.length;

    jsonData.forEach(game => {
        currentIssue++;
        // game image URL
        let imageSource;
        let cardType;
        let imageText = "N/A";
        let imageTextSize = 1.38;

        switch (true) {
            case game.image && avifSupport && game.type === "HB":
                imageSource = "https://obliteration.net/_images/HB/" + game.title + ".avif";
                break;
            case game.image && avifSupport:
                imageSource = "https://obliteration.net/_images/games/" + game.code + ".avif";
                break;
        }

        if (game.type === "HB") { // needs to be applied to all homebrews
            imageText = "HOME<br>BREW";
            imageTextSize = 1.25;
        } else if (game.type === "SYS") {
            imageText = "SYSTEM";
            imageTextSize = 1.13;
        }

        switch (true) {
            case totalIssues === 1 :
                cardType = "gameCard";
                break;
            case currentIssue === 1 :
                cardType = "gameCardS";
                break;
            case currentIssue === totalIssues :
                cardType = "gameCardE";
                break;
        }

        // game cards
        const gameElementHTML = `
    <a class="${cardType ? cardType : `gameCard`} ${game.tag}" target="_blank" href="https://github.com/obhq/compatibility/issues/${game.id}">
        ${imageSource ? `<img class="gameCardImage" loading="lazy" alt="${game.title} - ${game.code} game image" src="${imageSource}">` : `<p class='gameCardImageText' style='font-size: ${imageTextSize}rem;'>${imageText}</p>`}
        <div class="gameContent">
            <p class="gameCardTitle">${game.title}</p>
            <p class="gameCardCode">${game.code}</p>
            <p class="gameCardTag">${game.tag}</p>
            <p class="gameCardUpdated">${game.updated}</p>
        </div>
    </a>`;
        let cardClass = "." + (cardType ? cardType : `gameCard`);

        const tempContainer = document.createElement('a');
        tempContainer.innerHTML = gameElementHTML;
        let gameContainer = tempContainer.querySelector(cardClass);
        gameWrapper.appendChild(gameContainer);
    });

    document.getElementById("infoText").innerText = `${totalIssues} results`;
}