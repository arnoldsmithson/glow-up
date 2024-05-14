//https://www.quirksmode.org/js/cookies.html

function getThumbnailByUser() {
  if (readCookie("username")) {
    return `/thumbnails/${readCookie("username")}`;
  } else {
    return "/thumbnails/noname";
  }
}

function replaceContent(content) {
  document.getElementsByClassName("folder-button")[0].classList.remove('active')
  document.getElementsByClassName("folder-button")[1].classList.remove('active')
  document.getElementsByClassName("folder-button")[2].classList.remove('active')
  document.getElementById(content).classList.add('active');
  document.getElementById(content).setAttribute('hx-get')
}

function changeActive(name) {
  document.getElementsByClassName("active")[0].classList.remove("active");
  document.getElementById(name).classList.add("active");
}

function createCookie(name, value, days) {
  if (days) {
    var date = new Date();
    date.setTime(date.getTime() + days * 24 * 60 * 60 * 1000);
    var expires = "; expires=" + date.toGMTString();
  } else var expires = "";
  document.cookie = name + "=" + value + expires + "; path=/";
}

function readCookie(name) {
  var nameEQ = name + "=";
  var ca = document.cookie.split(";");
  for (var i = 0; i < ca.length; i++) {
    var c = ca[i];
    while (c.charAt(0) == " ") c = c.substring(1, c.length);
    if (c.indexOf(nameEQ) == 0) return c.substring(nameEQ.length, c.length);
  }
  return null;
}

function eraseCookie(name) {
  createCookie(name, "", -1);
}
