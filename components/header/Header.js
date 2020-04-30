import HeaderLink from './HeaderLink';
import Login from '../../components/login/Login';
import { useState } from 'react';

const Header = () => {
    const [login, showLogin] = useState(true);

    return (
        <nav className="navbar navbar-expand-md navbar-dark bg-primary mb-4">
            <div className="navbar-collapse collapse w-100 order-1 order-md-0 dual-collapse2">
                <ul className="navbar-nav mr-auto">
                    <HeaderLink name="Home" href="/" />
                    <HeaderLink name="Conferences" href="/conferences" />
                    <HeaderLink name="Reviews" href="/reviews" />
                </ul>
            </div>
            <div className="mx-auto order-0">
                <span className="navbar-brand mx-auto" href="#">Reorg</span>
                <button className="navbar-toggler ml-3" type="button" data-toggle="collapse" data-target=".dual-collapse2">
                    <span className="navbar-toggler-icon"></span>
                </button>
            </div>
            <div className="navbar-collapse collapse w-100 order-3 dual-collapse2">
                <ul className="navbar-nav ml-auto">
                    <HeaderLink name="About" href="/about" />
                    <HeaderLink name="User" href="/user" />
                    <Login />
                </ul>
            </div>
        </nav>
    );
}

export default Header;