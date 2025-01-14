class Widget {
    constructor() {
        this.ready = false; // @todo: Add initialization
    }

    render() {
        // @bug: Sometimes fails on Safari
        return '<div>Widget</div>';
    }
}
