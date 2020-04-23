import Container from 'react-bootstrap/Container';
import Header from '../components/header/Header';
import React, { Component } from 'react';

class Layout extends Component {
    render() {
        return (
            <Container>
                <Header />
                <Container>
                    {this.props.children}
                </Container>
            </Container>
        );
    }
}

export default Layout;