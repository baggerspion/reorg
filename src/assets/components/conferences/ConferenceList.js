import Conference from './Conference';
import gql from 'graphql-tag';
import { Query } from 'react-apollo';
import React, { Component } from 'react';
import Table from 'react-bootstrap/Table';

class ConferenceList extends Component {
    render() {
        return (
            <Query query={gql`
                {
                    allConferences {
                        data {
                            _id,
                            name,
                            short_name
                        }
                    }
                }
            `}>
                {({loading, error, data}) => {
                    if (loading) return <p>Loading...</p>;
                    if (error) return <p>Error: `${error.message}`</p>;
                    if (data) return (
                        <Table responsive striped bordered hover size="sm">
                            <thead>
                                <tr>
                                    <th>Event</th>
                                    <th style={{textAlign: "center"}}># Submissions</th>
                                    <th style={{textAlign: "center"}}># Reviewed</th>
                                </tr>
                            </thead>
                            <tbody>
                                {data.allConferences.data.map((event) => (
                                    <Conference conf={event} />
                                ))}
                            </tbody>
                        </Table>
                    );
                }}
            </Query>
        );
    }
}

export default ConferenceList;