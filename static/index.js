import init, { generate_mandelbrot_set } from "../pkg/wasm.js";
let imgWrapper;
let inputValues;
let submit;
let form;

const prepareDomElements = () => {
  form = document.querySelector(".form");
  imgWrapper = document.querySelector(".img__wrapper");
  inputValues = document.querySelectorAll(".form__input");
  submit = document.querySelector(".form__submit");
};

const getData = (event) => {
  event.preventDefault();

  let dataList = [];
  inputValues.forEach((element) => {
    dataList.push(Number(element.value));
  });
  validateData(dataList);
};

const validateData = (data) => {
  for (let i = 2; i < data.length; i++) {
    if (data[i] < -100 || data[i] > 100) {
      return fillImage(false, data);
    }
  }
  if (data[0] < 0 || data[1] < 0) {
    return fillImage(false, data);
  }
  if (data[2] > data[3] || data[4] > data[5]) {
    return fillImage(false, data);
  }
  return fillImage(true, data);
};

const generateMandelbrotData = async (data, canvas) => {
  
  canvas.width = data[1];
  canvas.height = data[1];
  generate_mandelbrot_set(data[0], data[1], data[2], data[3], data[4], data[5], canvas.getContext("2d"))
  console.log("Done!!!");
};

const fillImage = (response, data) => {
  while (imgWrapper.children.length != 0) {
    imgWrapper.removeChild(imgWrapper.children[0]);
  }

  if (response) {
    const newCanvas = document.createElement("canvas");
    newCanvas.classList.add("img__canvas");
    imgWrapper.appendChild(newCanvas);
    generateMandelbrotData(data, newCanvas);
  } else {
    const newMessage = document.createElement("h1");
    newMessage.classList.add("img__heading");
    newMessage.innerText = "Invalid Data!!!";
    imgWrapper.appendChild(newMessage);
  }
  form.reset();
};

const prepareDomEvents = () => {
  submit.addEventListener("click", getData);
};

const main = async () => {
  prepareDomElements();
  prepareDomEvents();
  await init();
};
main();
