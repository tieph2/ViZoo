import React from 'react';

const Header = () => {
    return (
        <div className="header">
            {/* Logo */}
            <div className="logo">
    <img src="path_to_your_logo_image" alt="Logo" />
        </div>

    {/* Search bar */}
    <div className="search-bar">
    <input type="text" placeholder="Search..." />
        <button>Search</button>
        </div>
        </div>
);
};

export default Header;