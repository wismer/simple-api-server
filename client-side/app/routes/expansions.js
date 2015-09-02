import Ember from 'ember';

export default Ember.Route.extend({
  model: function(params) {
    return this.store.findAll('expansion').then(function(res){
      debugger
    });
  }
});
