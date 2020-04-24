import Layout from '../../layouts/Layout';
import React, { Component } from 'react';

export default function conference(id) {
    const conference = id["url"]["query"]["id"]

    return (
        <Layout>
            <h1>Conference: {conference}</h1>
        </Layout>
    );
}