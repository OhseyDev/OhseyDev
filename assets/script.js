function toggleTheme() {
  let val = !localStorage.getItem("theme");
  localStorage.setItem("theme", val);
  document.body.classList.toggle("light");
  document.body.classList.toggle("dark");
}
document.addEventListener("ready", () => {
  let theme = localStorage.getItem("theme");
  if (theme) {
    document.body.classList.add("light");
    document.getElementById("dlswitch").checked = false;
  } else {
    document.body.classList.add("dark");
    document.getElementById("dlswitch").checked = true;
  }
});
function updateSwitch(self) {
  let theme = localStorage.getItem("theme");
  if (theme === undefined) {
    localStorage.setItem("theme", false);
    document.body.classList.add("dark");
  } else if (theme) { document.body.classList.add("light"); }
  else { document.body.classList.add("dark"); }
  self.checked = theme;
}
window.addEventListener("load", () => {
  var dlswitch = document.getElementById("dlswitch");
  updateSwitch(dlswitch);
  dlswitch.addEventListener("click", toggleTheme);
  document.getElementById("logo").addEventListener("click", () => {
    window.scrollTo(0, 0);
    window.location.href = "https://ohsey.me/";
  });
  if (document.body.id === "full") {
    window.addEventListener("scroll",()=>{document.body.style.setProperty("--scroll",(window.scrollY/window.innerHeight)*3)},false);
  }
});
