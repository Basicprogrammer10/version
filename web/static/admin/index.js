const TIME_UNITS = [
  ["second", 60],
  ["minute", 60],
  ["hour", 24],
  ["day", 30],
  ["month", 12],
  ["year", 0],
];

function getPassword(getpw) {
  let password = localStorage.getItem("password");
  if (password == null || getpw) {
    password = prompt("Enter password");
    localStorage.setItem("password", password);
  }
  return password;
}

async function init(getpw) {
  let password = getPassword(getpw);
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

async function initEdit() {
  let password = getPassword();
  let app = location.pathname.split("/")[3];
  let edit = await (
    await fetch(`/api/admin/edit/${app}`, {
      headers: {
        password: password,
      },
    })
  ).json();

  if ("error" in edit) {
    alert(`Error: ${edit.error}`);
    return;
  }

  return edit;
}

async function sumbitFileUpdate(versionId) {
  let access = document.querySelector("#access-code").value;
  let file = document.querySelector("#file-input-1").files[0];
  let bytes = new Uint8Array(await asyncFileReader(file));
  console.log(bytes);
}

function asyncFileReader(file) {
  return new Promise((resolve, reject) => {
    let reader = new FileReader();
    reader.readAsArrayBuffer(file);

    reader.onload = () => resolve(reader.result);
    reader.onerror = () => reject(reader.error);
  });
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