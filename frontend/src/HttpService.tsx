import axios from "axios";

const serverIP = import.meta.env.API_HOST;
const serverPort = import.meta.env.PORT;


export const serverUrl = `http://127.0.0.1:8080`;

// This is why I use Axios over Fetch
export const httpClient = axios.create({
    baseURL: serverUrl,
    headers: {
        "Content-type": "application/json",
    },
});



//Get id with only email
export async function getAnimalFromServer(species: any) {
    const get_id_config = {
        method: "post", // Specify your method here
        url: serverUrl + "/search",
        crossDomain: true,
        data: {
            species: species,
        },
    };
    const id = await httpClient.request(get_id_config);
    return id.data;
}

export async function getRandomAnimal() {
    const profile = await httpClient.get("/random");
    return profile.data;
}