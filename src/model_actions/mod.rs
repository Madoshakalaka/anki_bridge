/*
* The MIT License (MIT)
*
* Copyright (c) 2023 Codecrafter_404 <codecrafter_404@t-online.de>
*
* Permission is hereby granted, free of charge, to any person obtaining a copy
* of this software and associated documentation files (the "Software"), to deal
* in the Software without restriction, including without limitation the rights
* to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
* copies of the Software, and to permit persons to whom the Software is
* furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included in all
* copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
* IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
* FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
* AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
* LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
* OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
* SOFTWARE.
*/

/// Creates a new model to be used in Anki.
pub mod create_model;
/// Find and replace string in existing model by model name. Customise to replace in front, back or css by setting to true/false.
pub mod find_and_replace_in_models;
/// Creates a new field within a given model.
/// Optionally, the `index` value can be provided, which works exactly the same as the index in `modelFieldReposition`. By default, the field is added to the end of the field list.
pub mod model_field_add;
/// Gets the complete list of field descriptions (the text seen in the gui editor when a field is empty) for the provided model name.
pub mod model_field_descriptions;
/// Gets the complete list of fonts along with their font sizes.
pub mod model_field_fonts;
/// Gets the complete list of field names for the provided model name.
pub mod model_field_names;
/// Deletes a field within a given model.
pub mod model_field_remove;
/// Rename the field name of a given model.
pub mod model_field_rename;
/// Reposition the field within the field list of a given model.
/// The value of `index` starts at 0. For example, an index of `0` puts the field in the first position, and an index of `2` puts the field in the third position.
pub mod model_field_reposition;
/// Sets the description (the text seen in the gui editor when a field is empty) for a field within a given model.
/// Older versions of Anki (2.1.49 and below) do not have field descriptions. In that case, this will return with `false`.
pub mod model_field_set_description;
/// Sets the font for a field within a given model.
pub mod model_field_set_font;
/// Sets the font size for a field within a given model.
pub mod model_field_set_font_size;
/// Returns an object indicating the fields on the question and answer side of each card template for the given model name. The question side is given first in each array.
pub mod model_fields_on_templates;
/// Gets the complete list of model names for the current user.
pub mod model_names;
/// Gets the complete list of model names and their corresponding IDs for the current user.
pub mod model_names_and_ids;
/// Gets the CSS styling for the provided model by name.
pub mod model_styling;
/// Adds a template to an existing model by name. If you want to update an existing template, use `updateModelTemplates`.
pub mod model_template_add;
/// Removes a template from an existing model.
pub mod model_template_remove;
/// Renames a template in an existing model.
pub mod model_template_rename;
/// Repositions a template in an existing model.
/// The value of `index` starts at 0. For example, an index of `0` puts the template in the first position, and an index of `2` puts the template in the third position.
pub mod model_template_reposition;
/// Returns an object indicating the template content for each card connected to the provided model by name.
pub mod model_templates;
/// Modify the CSS styling of an existing model by name.
pub mod update_model_styling;
/// Modify the templates of an existing model by name. Only specifies cards and specified sides will be modified. If an existing card or side is not included in the request, it will be left unchanged.
pub mod update_model_templates;
