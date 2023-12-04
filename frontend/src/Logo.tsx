import React from 'react';
import logo from "./assets/zoo.jpg"

const Logo = () => {
    return (
        <div className="header">
            <div className="logo">
                <img src={logo} alt="Logo" />
            </div>
        </div>
);
};

export default Logo;