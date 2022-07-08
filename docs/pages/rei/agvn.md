---
layout: default
title: Automated Governing System
parent: Rei
---

How would we implement AGVN System v1 in rei?

```rust
// server.rei

@route('/budget')
fn get_budget() -> Budget {
    return current_budget
}

@route('/budget/next_cycle')
fn get_next_budget() -> Budget {
    return next_budget
}

@route('/info')
fn get_general_info() -> rei::any {
    return government_info
}

// this pickles and formats users into JSON and sends a HTTP reply with its body
@route('/users')
fn get_users() -> List<User> {
    return global.users
}

@protected_route('/update')
fn update() {
    // updates the state, kind of like a 'tick' that writes pending changes to the main database
    pending_updates = auxiliary_db.changes()
    main_db.make_change_request(pending_updates)
    // basically migrate
    main_db.apply_changes()
}

// all functions with 'aux' keyword are deemed 'supporting' functions
// i.e. they may be inlined and optimised in different ways to 'key' functions
aux fn start_data_processing() {
    // start data processing pipeline
    data_pipeline.start()
}

// should be used when the pending data batch size >= threshold or time_elapsed >= 5.0 days, generally 500 data points for threshold
aux fn fit_data(data: &TrainData) {
    // fits data to the main neural network (retrains the neural net)
    // there should be at least 1 data point
    __core_neural_net.fit(data)
}

aux fn view_model(output_framebuffer: *FrameBuffer) {
    output_framebuffer = __core_neural_net.render()
}

// should be used on user data to partition them based on commonalities
// that the model can exploit to ensure no one group is getting too big
// and ensure a low distance between each group, and many groups
aux fn cluster_data(data: &List<UserData>) {
    __core_cluster.fit(data)
    // if anomalies detected, warn the neural network and government subsystems
    let possible_anomalies: ClusterAnomalies = __core_cluster.anomalies()

    // overload operator() to test for at least one anomaly
    if possible_anomalies() {
        government.social_department.warn(Warning::Anomaly, possible_anomalies)
    }
}

// in government.rei

// 3-dimensional political cube
object PoliticalAlignment {
    @range(-1,1) {
        libertarianist_index: f64,
        leftist_index: f64,
        enlightenment_index: f64,
    }    
}

// Government's job is to make policy decisions and budget changes on the fly
// should try to update them on the fly when something happens like:
// emotional changes within the population, global or domestic event -> predict outcomes
// and big changes when they get elected
class Government {
    political_alignment: PoliticalAlignment

    // if there is a revolution, the whole thing will have to crumble
    // on a new election, a new government can be formed
    constructor(political_alignment: PoliticalAlignment) {
        self.political_alignment = political_alignment
    }

}

// Things a government shouldnt have access to, to ensure the government doesnt become tyrannical
class GovernmentSupervisor {

    constructor() {}

    @export
    fn hold_election() {
        self.__start_election()
    }

    @not-implemented
    fn __start_election() {}
}

class ForeignRelationsDepartment {
    
    constructor() {}

    // can only interact with enlightened governments
    // who implement a digital EnlightenedGovernment
    fn interact_with(governments: &List<EnlightenedGovernment>) {
        governments.map(g -> self.negotiate_with(g))
    }

    @not-implemented
    async fn negotiate_with(government: EnlightenedGovernment) {}
}

struct WorldEvent {
    details: String,
    approx_time_began: Timestamp,
    ongoing: bool,
    approx_time_end: Timestamp,
}

class PublicRelationsDepartment {

    constructor() {}

    @protected_route('/statements')
    fn make_public_statement(event: WorldEvent) {
        let statement = __core_nlp_server.request_statement(event)
        return statement
    }
}

// federal level emergencies
class EmergencyDepartmentSupervisor {

    constructor() {}

    pending_messages: List<Message>

    // a PR statement should be made by the PR deparment
    fn tell_pr_department(emergency: Emergency) {
        let event = emergency.to_event()
        pr_department.make_public_statement(event)
    }

    @protected_route('/messages')
    fn get_messages() {
        return self.messages
    }

    // tell workers to fix the problem
    fn send_messages(workers: List<Workers>) {
        workers.map(w -> send_message(w))
    }
}

struct Army {
    n_personnel: Size,
    @positive
    power_level: f64,
    tools: List<MilitaryTool>
}

struct Region {
    bounding_area: BoundingArea,
    controlled_by: List<String>
}

enum DeployType {
    FULL, LIMITED
}

class DefenseDepartment {
    constructor() {}

    defense_forces: List<Army>

    // update the core neural network of the global military and conflict status
    fn update_conflicts_status(current_conflict_status: &WorldConflictStatus) {
        __core_neural_network.update_conflicts(current_conflict_status)
    }

    // begin invasion of a region
    fn invade(region: &Region) {
        let result = __core_neural_network.request_invasion(region)
        deploy(...result)
    }

    fn defend(region: &Region) {
        let result = __core_neural_network.request_defense(region)
        deploy(...result)
    }

    fn deploy(coordinates: [f64, f64], deploy_type: &DeployType) {
        workers.send_message(Message::DeployMilitary, deploy_type, coordinates)
    }
}

class EducationDepartment {
    constructor() {}

    state_education_departments: List<StateEducationDepartment>

    fn push_new_education_policy() {
        let result = __core_neural_network.get_education_policy()
        self.state_education_departments.update_policy(...result)
    }
}

// in neural_network.rei

// object constructor
NeuralNode = {
    connections: List<&Node>
}

NeuralNetworkState = {
    nodes: List<Node>
}

// ! contains almost everything you can imagine
// HUMAN INTERNATIONAL
// HDI2, EI, healthcare status, emotion status, conflict status
// education status, trade and economic status (inflation, etc)
// on both federal, state and local levels. If a country has more or less levels, account for that with Levels 0-N-1
// HUMAN DOMESTIC
// same stuff, but weighted higher, maybe 0.75
// HUMAN INDIVIDUAL
// try to analyse as many humans as possible. Put more weight, e.g. 0.75 on domestic individuals
// basics: age, height, sex etc. If possible include the entire phenotype (fully textured 3d model) and genotype (DNA scan)
// income, education level, relationships with other humans, political beliefs and philosophy, etc.
// UNIVERSAL
// position of the earth and the moon
// position of the solar system in the galaxy and the galaxy relative to other galaxies in the local cluster
class WorldData {
    known_individuals: List<IndividualData>
    domestic_info: DomesticData
    international_info: InternationalData
    universe_info: UniverseData

    constructor() {}
}

// needs to be able to self label and predict based on past data
// the idea is to generate a really, really big model that can predict almost anything in the universe
// esp humans
// Maybe just start off with human civilisations from 0-12000
// not much data before 5000BC though, and more skewed towards recent (100-200 years), western data
// doesnt really matter, that just means the current way of living is maximal so all humans should try to live like this
class NeuralNetwork {
    constructor() {}

    // actual model
    model: NeuralNetworkModel

    // mostly for viewing
    nn_state: NeuralNetworkState

    fn fit(data: &WorldData) {
        model.train(data)
        model.validate()
    }

    // predict a value of a class based on inputs
    // assumes you mean now
    @export
    fn predict(inputs: &WorldDataInput, predict_label: &String) -> WorldDataPrediction {
        let prediction = model.query(inputs)
        return prediction
    }

    // possible to predict multiple classes at once at a certain time
    @not-implemented
    fn predict_multiple() {}

    // for internal use only
    @not-implemented
    fn validate_predict() {}

    // specialised method to fit a lot on a lot of data
    // e.g. when doing so for the first time on the entire human history
    @not-implemented
    fn full_fit(data: &WorldData) {}

    // NOTE: the neural network is meant to predict upcoming statistics within a 20 year timeframe
    // it is trained to be able to self label each feature and predict that over a 12000 year timeframe
    // then the departments can use predicted data within 20 years to make a mid-long term decision or general policy
    // if there is a huge event that causes some problem like a war or famine, then this should be fed as an event or possibly included in worlddata. The network can be updated with a new set of worlddata each day
    // departments like the emergency department and the war department should also consult with the main neural network for a general policy
    // if a huge event happens, they will probably need to act asap. So they should have priority in their queries to the central neural network
    // but they should also be somewhat independent. These departments will have their own smaller neural networks that take in emergency/war events and analyse how things played out. And which actions led to which consequences and optimise for the good consequences
}
```
