const BASE_URL = "https://prd.moe/s/indexes/"
const PUBLIC_API_KEY = "10fa4ee0bbc884ff6aab854093a682083dc33c054cf202e09f074257a3dd01c5";
function httpGetAsync(callback, input) {
    let generalSearchUrl = new URL(BASE_URL + 'all/search');
    generalSearchUrl.searchParams.set("q", input);
    let xmlHttp = new XMLHttpRequest();
    xmlHttp.onreadystatechange = () => {
        if (xmlHttp.readyState === 4 && xmlHttp.status === 200) callback(xmlHttp.responseText);
    }
    xmlHttp.open("GET", generalSearchUrl, true);
    xmlHttp.setRequestHeader('Authorization', `Bearer ${PUBLIC_API_KEY}`);
    xmlHttp.send();
}

const handleSearch = (results) => {
    let content = document.getElementById("content");
    let newContent = document.getElementById("searchcontent");
    results = JSON.parse(results);
    const hits = results.hits;

    if (results.query === "") {
        if (newContent) document.body.removeChild(newContent);
        content.removeAttribute("style");
        return;
    }

    content.style.display = "none";
    if (!newContent) {
        newContent = document.createElement('div');
        newContent.setAttribute("id", "searchcontent");
        document.body.appendChild(newContent);
    }

    newContent.innerHTML = "";
    for (const hit of hits) {
        let result = document.createElement("div");
        result.setAttribute("class", "searchresult");
        result.innerHTML = hit.content;
        newContent.appendChild(result);
        newContent.appendChild(document.createElement("hr"));
    }
}
