import React from 'react';

// @ts-ignore
const AnimalCard = ({ searchResult }) => {
    return (
        <div className=" card bg-white rounded-lg p-4 shadow-md">
            <h2 className="text-xl font-bold mb-4">Animal Details</h2>
            <img src={searchResult.img_url} alt="Animal" className="animal-thumb mb-4 rounded-lg" />
            <p className="mb-2"><strong>Name:</strong> {searchResult.name}</p>
            <p className="mb-2"><strong>Scientific Name:</strong> {searchResult.scientific_name}</p>
            <p className="mb-2"><strong>Location:</strong> {searchResult.locations}</p>
            <p className="mb-2"><strong>Prey:</strong> {searchResult.prey}</p>
            <p className="mb-2"><strong>Diet:</strong> {searchResult.diet}</p>
            <p className="mb-2"><strong>Life span:</strong> {searchResult.lifespan}</p>
            <p className="mb-2"><strong>Weight:</strong> {searchResult.weight}</p>
        </div>
    );
};

export default AnimalCard;