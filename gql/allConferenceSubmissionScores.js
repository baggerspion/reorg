import gql from 'graphql-tag';

const allConferenceSubmissionScores = (short_name) => {
    return (
        gql`query FindAllSubmissionScores {
            conferenceByShortName(short_name: "${short_name}") {
                submissions {
                    data {
                        title
                        reviews {
                            data {
                                score
                            }
                        }
                    }
                }
            }
        }`
    );
}

export default allConferenceSubmissionScores;