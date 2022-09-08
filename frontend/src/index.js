import {counter} from "../../declarations/counter"

document.querySelector("form").addEventListener("submit", async (e) => {
    e.preventDefault();
    const button = e.target.querySelector("button");

    const name = document.getElementById("name").value.toString();

    button.setAttribute("disabled", true);

    // Interact with foo actor, calling the greet method
    await  counter.increment();
    const greeting = await counter.get();
    console.log(greeting)
    button.removeAttribute("disabled");

    document.getElementById("greeting").innerText = greeting;

    return false;
});
