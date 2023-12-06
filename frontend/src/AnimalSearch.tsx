import React, {useEffect, useState} from 'react';
import {getAnimalFromServer, getRandomAnimal} from './HttpService'
import AnimalCard from "./AnimalCard";

const AnimalSearch = () => {
    const [searchTerm, setSearchTerm] = useState('blue whale');
    const [searchResults, setSearchResults] = useState([]);

    const fetchAnimal = async () => {
        console.log("Calling API")
        await getAnimalFromServer(searchTerm)
            .then((response) =>
            {
                console.log("Got animal", response);
                setSearchResults(response)
            })
            .catch((err) => console.log("Error in fetch animals", err));
    };

    const fetchRandom = async () => {
        console.log("Calling API")
        await getRandomAnimal()
            .then((response) =>
            {
                console.log("Got animal", response);
                setSearchResults(response)
            })
            .catch((err) => console.log("Error in fetch animals", err));
    };
    useEffect(() => {
        (async () => {
            await fetchAnimal();
        })();
    }, []);


    return (
        <div>
            <div className="flex">
                <input
                    type="text"
                    value={searchTerm}
                    onChange={(e) => setSearchTerm(e.target.value)}
                    placeholder="Search for an animal..."
                    className="custom-input"

                />
                <button onClick={fetchAnimal} className="custom-button">
                    Search
                </button>
                <button onClick={fetchRandom} className="custom-button">
                    Random!
                </button>
            </div>

            <div className={"result"}>
                {searchResults ? searchResults.map(item => (
                    <AnimalCard searchResult={item} />
                    )) : (
                    <div> No animal found </div>
                    )}
            </div>
        </div>
    );
};

export default AnimalSearch;