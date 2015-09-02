import Ember from 'ember';
import config from './config/environment';

var Router = Ember.Router.extend({
  location: config.locationType
});

Router.map(function() {
  this.route('cards');
  this.route('expansions', { path: '/expansions' }, function() {
    this.route('expansion', { path: ':expansion_slug' });
  });
});

export default Router;
