import Ember from 'ember';

export default Ember.Route.extend({
  model: function(params) {
    return this.store.findBy('slug', params.expansion_slug);
  },

  serialize: function(model) {
    return { expansion_slug: model.get('slug') };
  }
});
