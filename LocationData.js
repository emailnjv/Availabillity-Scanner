const blocks = document.querySelectorAll("#locationsDiv > div.text-capitalize");

const licensingLocationMap = {};

blocks.forEach(i => {
  const splitByLine = i.innerText.split("\n\n")[0].split("\n");

  const locationTitle = i.innerText.split(" - ")[0];
  const locationStreet = splitByLine.slice(1, splitByLine.length - 1).join(", ")
  const locationCityZip = splitByLine[splitByLine.length - 1];
  const locationTown = locationCityZip.split(",")[0]
  const locationZip = locationCityZip.split("NJ ")[1]
  const locationID = parseInt((i.querySelector("a[href*='getFirstDate']").href.split("(")[1]).split(",")[0])

  licensingLocationMap[locationID] = {
    locationID,
    locationTitle,
    locationStreet,
    locationTown,
    locationZip
  }
})

console.log(licensingLocationMap);