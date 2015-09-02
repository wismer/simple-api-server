import DS from 'ember-data';

export default DS.Model.extend({
  expansion: DS.belongsTo('expansion')
});
