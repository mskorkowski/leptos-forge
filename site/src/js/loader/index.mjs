function showProgress() {
  document.getElementById("loader-container").style.display = "flex";
  document.getElementById("loader").style.display = "block";
}

function updateProgress(percent) {
  console.log("Loading... "+percent+"%");
  document.getElementById("loader-progress").style.width = ""+percent+"%";
}

function loaded() {
  document.getElementById("loader-container").remove();
}

function failed(error) {
  let errorContainer = document.getElementById("loader-error");
  let text = document.createTextNode(error);
  errorContainer.appendChild(text);
}

export default function initializer () {
  return {
    // called when the initialization starts 
    onStart: () => {
        showProgress();
    },
    // called when the download progresses, including a last update at the ennd
    onProgress: ({current, total}) => {
        updateProgress(Math.round((current / total) * 100));
    },
    // called when the process is complete (successful or not) 
    onComplete: () => {},
    // called when the process has completed successfully, including the WebAssembly instance
    onSuccess: (wasm) => {
        loaded();
    },
    // called when the process failed, including the error 
    onFailure: (error) => {
        failed(error);
    }
  }
};