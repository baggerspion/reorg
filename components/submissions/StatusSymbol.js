const StatusSymbol = (props) => {
    switch (props.status) {
        case "SUBMITTED":
            return ("📃");
        case "SELECTED":
            return ("✅");
        case "REJECTED":
            return ("❌");
        case "WAITING":
            return ("⏳");
    }
}

export default StatusSymbol;