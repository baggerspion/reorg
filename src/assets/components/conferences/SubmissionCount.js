import gql from 'graphql-tag';
import { Query } from 'react-apollo';
import React, { Component } from 'react';

class SubmissionCount extends Component {
    render() {
        return (
            <Query query={gql`
                {
                    findConferenceByID(id: ${this.props.event}) {
                        submissions {
                            data {
                                _id
                            }
                        }
                    }
                }         
            `}>
                {({loading, error, data}) => {
                    if (loading) return <span>Loading...</span>;
                    if (error) return <span>Error</span>;
                    if (data) return <span>{data.findConferenceByID.submissions.data.length}</span>;
                }}
            </Query>
        );
    }
}

export default SubmissionCount;