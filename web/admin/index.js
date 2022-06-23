const TIME_UNITS = [
  ["second", 60],
  ["minute", 60],
  ["hour", 24],
  ["day", 30],
  ["month", 12],
  ["year", 0],
];

async function init() {
  let password = localStorage.getItem("password");
  if (password == null) {
    password = prompt("Enter password");
    localStorage.setItem("password", password);
  }

  let home = await (
    await fetch("/api/admin/home", {
      headers: {
        password: password,
      },
    })
  ).json();

  if ("error" in home) {
    alert(`Error: ${home.error}`);
    return;
  }

  return home;
}

function epochTime(time) {
  time = new Date().getTime() / 1000 - time;
  for (let e = 0; e < TIME_UNITS.length; e++) {
    const i = TIME_UNITS[e];

    if (i[1] == 0 || time < i[1]) {
      time = Math.round(time);
      return `${time} ${i[0]}${time > 1 ? "s" : ""} ago`;
    }

    time /= i[1];
  }

  return `${Math.round(time)} years ago`;
}
